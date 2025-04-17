#!/bin/bash

# Exit on any error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting ServerWatch Agent installation...${NC}"

# Check for required tools
command -v curl >/dev/null 2>&1 || { echo -e "${RED}curl is required. Please install it (e.g., 'sudo apt install curl').${NC}"; exit 1; }
command -v openssl >/dev/null 2>&1 || { echo -e "${RED}OpenSSL is required. Please install it (e.g., 'sudo apt install openssl').${NC}"; exit 1; }

# Prompt for password
echo "Please enter the password for ServerWatch Agent:"
read -s SERVERWATCH_PASSWORD
if [ -z "$SERVERWATCH_PASSWORD" ]; then
    echo -e "${RED}Password cannot be empty. Exiting.${NC}"
    exit 1
fi

# Set install directory
INSTALL_DIR="$HOME/ServerWatch-agent"
mkdir -p "$INSTALL_DIR"
cd "$INSTALL_DIR"

# Download the pre-built binary
echo "Downloading ServerWatch-Agent binary..."
curl -sSL -o ServerWatch-Agent https://raw.githubusercontent.com/AlexanderHeffernan/ServerWatch/main/agent/bin/ServerWatch-Agent
chmod +x ServerWatch-Agent

# Generate unique self-signed certificate and key
if [ ! -f "serverwatch.crt" ] || [ ! -f "serverwatch.key" ]; then
    echo "Generating self-signed certificate and key..."
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout serverwatch.key \
        -out serverwatch.crt \
        -subj "/C=US/ST=YourState/L=YourCity/O=ServerWatch/OU=Agent/CN=$(hostname)" \
        -addext "subjectAltName=DNS:$(hostname)"
    chmod 600 serverwatch.crt serverwatch.key
fi

# Set up systemd service
SERVICE_FILE="/etc/systemd/system/serverwatch-agent.service"
echo "Configuring systemd service..."
sudo bash -c "cat > $SERVICE_FILE" <<EOL
[Unit]
Description=ServerWatch Agent
After=network.target

[Service]
ExecStart=$INSTALL_DIR/ServerWatch-Agent
WorkingDirectory=$INSTALL_DIR
Environment="SERVERWATCH_PASSWORD=$SERVERWATCH_PASSWORD"
Restart=always
User=$(whoami)

[Install]
WantedBy=multi-user.target
EOL

# Reload and enable service
sudo systemctl daemon-reload
sudo systemctl enable serverwatch-agent.service
sudo systemctl start serverwatch-agent.service

# Verify it's running
if sudo systemctl is-active serverwatch-agent.service >/dev/null; then
    echo -e "${GREEN}ServerWatch Agent installed and running at https://$(hostname):49160/metrics${NC}"
    echo "Default password is 'Password123'. Test with:"
    echo "curl --insecure https://127.0.0.1:49160/metrics?password=Password123"
else
    echo -e "${RED}Failed to start ServerWatch Agent. Check logs with 'journalctl -u serverwatch-agent.service'${NC}"
    exit 1
fi

echo -e "${GREEN}Installation complete! Visit https://alexanderheffernan.github.io/ServerWatch/ to connect.${NC}"