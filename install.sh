#!/bin/bash
set -euo pipefail
umask 077

# Build on the target host (or pass a binary built for its architecture).
# The historical agent/bin binary implements the old, unsafe wire protocol.
BINARY="${1:-$(dirname "$(readlink -f "$0")")/agent/target/release/ServerWatch-Agent}"
if [ ! -f "$BINARY" ]; then
    echo "Build the reviewed source first: cargo build --release --locked --manifest-path agent/Cargo.toml"
    echo "Usage: ./install.sh /path/to/new/ServerWatch-Agent"
    exit 1
fi
if systemctl cat serverwatch-agent.service >/dev/null 2>&1; then
    echo "Existing service detected. Follow the upgrade instructions in readme.md; this installer is for new installs."
    exit 1
fi
for tool in openssl sudo visudo useradd install; do
    command -v "$tool" >/dev/null || { echo "$tool is required"; exit 1; }
done
read -r -s -p "Administrator password (use a unique random secret): " PASSWORD
echo
[ -n "${PASSWORD//[[:space:]]/}" ] || { echo "Password cannot be empty"; exit 1; }
read -r -s -p "Alertzy account key (optional): " ALERTZY_KEY
echo

STAGING=$(mktemp -d)
trap 'rm -rf "$STAGING"' EXIT
# Quote EnvironmentFile values without shell evaluation or systemd specifier expansion.
write_env() {
    local value=${2//\\/\\\\}
    value=${value//\"/\\\"}
    printf '%s="%s"\n' "$1" "$value"
}
{
    write_env SERVERWATCH_PASSWORD "$PASSWORD"
    write_env SERVERWATCH_ALERTZY_KEY "$ALERTZY_KEY"
    write_env SERVERWATCH_BIND "127.0.0.1:49160"
    write_env SERVERWATCH_ALLOWED_ORIGINS "https://serverwatch.alexheffernan.dev"
} > "$STAGING/agent.env"
unset PASSWORD ALERTZY_KEY

if id serverwatch >/dev/null 2>&1; then
    if [ "$(id -u serverwatch)" -eq 0 ] || [ "$(id -nG serverwatch)" != serverwatch ] || \
        [ "$(getent passwd serverwatch | cut -d: -f6-)" != /var/lib/serverwatch:/usr/sbin/nologin ]; then
        echo "Existing serverwatch account is not the isolated service account; review it before installing."
        exit 1
    fi
else
    sudo useradd --system --user-group --home-dir /var/lib/serverwatch --shell /usr/sbin/nologin serverwatch
fi
sudo install -d -o root -g root -m 755 /opt/serverwatch
sudo install -o root -g root -m 755 "$BINARY" /opt/serverwatch/ServerWatch-Agent
sudo install -d -o root -g serverwatch -m 750 /etc/serverwatch
sudo install -o root -g root -m 600 "$STAGING/agent.env" /etc/serverwatch/agent.env
sudo install -d -o serverwatch -g serverwatch -m 700 /var/lib/serverwatch
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout "$STAGING/serverwatch.key" -out "$STAGING/serverwatch.crt" \
    -subj "/CN=$(hostname)" -addext "subjectAltName=DNS:$(hostname)"
sudo install -o root -g serverwatch -m 640 "$STAGING/serverwatch.key" "$STAGING/serverwatch.crt" /etc/serverwatch/
sudo ln -s /etc/serverwatch/serverwatch.key /var/lib/serverwatch/serverwatch.key
sudo ln -s /etc/serverwatch/serverwatch.crt /var/lib/serverwatch/serverwatch.crt

# No general-purpose sudo, shell, or argument wildcards. Validate before installing.
echo 'serverwatch ALL=(root) NOPASSWD: /usr/sbin/shutdown -h now, /usr/sbin/reboot ""' > "$STAGING/serverwatch-power"
sudo visudo -cf "$STAGING/serverwatch-power"
sudo install -o root -g root -m 440 "$STAGING/serverwatch-power" /etc/sudoers.d/serverwatch-power
cat > "$STAGING/serverwatch-agent.service" <<'EOF'
[Unit]
Description=ServerWatch Agent
After=network.target

[Service]
ExecStart=/opt/serverwatch/ServerWatch-Agent
WorkingDirectory=/var/lib/serverwatch
EnvironmentFile=/etc/serverwatch/agent.env
User=serverwatch
Group=serverwatch
Restart=on-failure
RestartSec=5
UMask=0077
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/serverwatch
PrivateTmp=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
# NoNewPrivileges is deliberately not enabled: the two exact sudo power commands need setuid.

[Install]
WantedBy=multi-user.target
EOF
sudo install -o root -g root -m 644 "$STAGING/serverwatch-agent.service" /etc/systemd/system/serverwatch-agent.service
sudo systemctl daemon-reload
sudo systemctl enable --now serverwatch-agent.service
echo "Installed with loopback-only HTTPS. Configure private-network/proxy access and trusted TLS per readme.md before connecting."
