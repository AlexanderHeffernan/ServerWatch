# ServerWatch

ServerWatch is a lightweight Vue dashboard and Rust HTTPS agent for CPU, RAM,
disk and temperature metrics, temperature alerts, and server power controls.
The agent has only been tested on Raspberry Pi OS. Other platforms require
validation, especially sensor visibility and power-command permissions.

## Background

I created ServerWatch after setting up a Raspberry Pi 5 as a personal projects
server. I needed a simple way to check its performance remotely—CPU load,
temperatures, etc.—and built this tool to fill that gap.

## Security and deployment

**Treat this as an administrative service, not a public metrics endpoint.**
Use a private LAN/VPN or an authenticated reverse proxy such as Cloudflare
Access. Do not expose port 49160 with public port forwarding. CORS is a browser
restriction, not a firewall or an alternative to authentication.

- The agent defaults to `127.0.0.1:49160` over HTTPS. For direct LAN/VPN use,
  explicitly set `SERVERWATCH_BIND` to the private interface and port, restrict
  inbound traffic to trusted clients, and use a certificate trusted by their
  browsers. Do not disable certificate verification as a deployment solution.
- `SERVERWATCH_PASSWORD` is a required, nonempty **administrator** secret. Use
  a unique random password (at least 32 random characters recommended).
  Rotate any password previously sent in URLs: historical browser, proxy and
  server logs may contain it. Never log Authorization headers.
- Optional `SERVERWATCH_READ_PASSWORD` must be nonempty and different from the
  admin secret. It permits connection checks, metrics and reading temperature
  settings, but not configuration writes, shutdown or reboot. Enter either
  password in the existing connection form. Denied actions leave the session
  connected and add an error to the existing notification menu.
- `SERVERWATCH_ALLOWED_ORIGINS` is a comma-separated list of exact dashboard
  origins, without paths/trailing slashes. Default:
  `https://serverwatch.alexheffernan.dev`. Set this for self-hosted dashboards;
  use an empty value to disallow cross-origin browser access. Wildcards and
  `null` origins are not allowed. The allowed origin's JavaScript can use the
  password you enter, so only trust dashboards you control.
- Requests use `Authorization: Bearer <password>`, never query passwords or
  cookies for agent authentication. All API responses are marked `no-store`.
  A peer-IP rate limiter allows a burst of 10 requests and replenishes one
  every 3 seconds. Proxied clients share the proxy's peer bucket; forwarded
  headers are deliberately not trusted. Add per-user limits at the proxy for
  larger installations; do not expose development servers to the Internet.

### Cloudflare Access / reverse proxy

For browser-based Access authentication, serve a self-hosted dashboard and
proxy the six API paths below on **the same HTTPS origin**, protected by one
Access application. Log in through Access before connecting, then enter that
origin's hostname in ServerWatch. Same-origin requests retain the Access
session cookie and still require the agent bearer password. A public dashboard
calling a different Access-protected origin is not a supported cookie-login
flow (cross-origin cookies are intentionally not sent). Do not create Access
bypass policies for the API to make that topology work.

Keep the agent on loopback behind the tunnel/proxy; verify the upstream agent
certificate using a trusted CA/pinned origin certificate. Do not use
`noTLSVerify`. Configure the proxy-to-agent connection as **HTTP/1.1**, including
disabling HTTP/2-to-origin where applicable. Restrict direct origin access so
clients cannot bypass Access. Proxy OPTIONS requests without requiring the
agent bearer header (preflights do not contain it); do not bypass Access on
actual API requests. No infrastructure is configured by this repository.

### API and coordinated upgrade

Upgrade agent and frontend together. Old query-password clients are deliberately
incompatible and receive 401; GET power/configuration mutations receive 405.

| Method | Path | Permission |
| --- | --- | --- |
| GET | `/test-connection` | Read or admin |
| GET | `/metrics` | Read or admin |
| GET | `/get-temp-config` | Read or admin |
| PUT | `/set-temp-config` | Admin; JSON body |
| POST | `/shutdown` | Admin |
| POST | `/reboot` | Admin |

Configuration JSON retains `warning_temp`, `shutdown_temp`, `warnings_enabled`
and `shutdown_enabled`. Temperatures must be integer °C in 40–150, with warning
strictly below shutdown, even when an action is disabled. Writes are atomic;
invalid input returns 400 and persistence errors return 500 without claiming
success. Missing/malformed/out-of-range on-disk settings use the existing safe
defaults (75/85°C, both enabled) and log a warning rather than killing the
thermal monitor. Check existing settings before upgrading.

Metrics are sampled in a blocking background job approximately every three
seconds, not once per request. The initial sampling window can return 503;
refresh shortly afterward. The JSON shape is unchanged. Thermal monitoring
runs every 30 seconds on the same long-lived runtime as the server and retries
if a critical-temperature shutdown did not stop the host. Alertzy is optional:
notifications have a 1-second connection / 2-second total timeout, and power
actions proceed after notification failure or timeout. The native command uses
noninteractive sudo; permission/command failures return 500.

## Build and install

### Frontend

Use Node 24 LTS (Node 22.12+ also supported) and npm 11. The obsolete Vue CLI /
Webpack build chain was replaced with Vite to remove unpatched tooling
advisories, without changing component layouts or styles.

```sh
npm ci --prefix frontend
npm run lint --prefix frontend
npm test --prefix frontend
npm run build --prefix frontend
```

Serve `dist/` with a production static server, or use the existing Dockerfile
(Node 24 build stage and nginx). Development: `npm run serve --prefix frontend`.
The repository-root static files are a generated snapshot for existing static
hosting; regenerate them when releasing frontend changes. The service worker
keeps the existing `service-worker.js` URL and offline shell, does not cache
API requests, and retains the existing `site.webmanifest` and icons. Refresh
and close/reopen old tabs to activate a newly installed worker during upgrade.

### Agent (Linux/systemd)

Install a current stable Rust toolchain and native C/C++ build tools (including
CMake for TLS dependencies). Build for the target server's architecture:

```sh
cargo build --release --locked --manifest-path agent/Cargo.toml
# Review the script before running it; this installs and starts a host service.
bash install.sh /path/to/new/ServerWatch-Agent
```

The installer no longer downloads an unverifiable, stale ARM binary from the
repository. It takes your newly built binary. It creates a locked `serverwatch`
service account, a root-owned executable in `/opt/serverwatch`, root-owned
credentials/TLS files in `/etc/serverwatch`, and writable configuration state
in `/var/lib/serverwatch`. Passwords are kept in root-only `agent.env`, not the
world-readable unit. Edit that file with a privileged editor for optional
read-only credentials, bind address and CORS origins, then restart the service.

The unit restricts filesystem writes and home/kernel access. The account gets
only these exact passwordless sudo commands: `/usr/sbin/shutdown -h now` and
`/usr/sbin/reboot` with no arguments. No general sudo or sudo-group membership is
needed. `NoNewPrivileges` is deliberately not set because these commands need
sudo's setuid transition. Review and verify command paths, sudo policy and
sensor visibility on the actual target OS; do not run the service as root or as
your personal sudo-capable login. A compromised agent can still power off the
host by design; this is why network isolation and read/admin separation matter.

For an **existing installation**, schedule a maintenance window and back up
the old unit, certificates and `config.json`. Stop/disable the old service and
remove its unit file, then reload systemd before using the new-install script
(it refuses to overwrite an existing service). Copy the validated old config
to `/var/lib/serverwatch/config.json`, owned by `serverwatch:serverwatch`, mode
600. Restore trusted certificates as root-owned, group-readable files in
`/etc/serverwatch` if appropriate. Set the new credentials/origins/bind, remove
obsolete broad sudo grants belonging only to the old installation, and restart
the new service. Rotate the old password. Keep backups until connection,
metrics and configuration have been checked. Real host power tests require a
separately approved maintenance window; they are never part of automated tests.

Uninstall: review and run `bash uninstall.sh`. This removes the service,
new-install files/state and its exact sudoers rule (and the legacy
`~/ServerWatch-agent` directory if present); it retains the locked account to
avoid reassigning old file ownership.

## Verification and remaining advisory

```sh
env -u SERVERWATCH_ALERTZY_KEY cargo test --locked --manifest-path agent/Cargo.toml
cargo clippy --locked --manifest-path agent/Cargo.toml --all-targets -- -D warnings
cargo audit --file agent/Cargo.lock
npm audit --prefix frontend
bash -n install.sh uninstall.sh
```

Backend route tests inject harmless power execution, exercise denied/allowed
requests and configuration persistence, and test notification errors/timeouts
without invoking host shutdown/reboot. Frontend tests cover the matching wire
contract, special-character secrets, denied actions and request timeouts.

As of the security refresh, npm audit reports **zero** affected entries. Rust
audit still reports **RUSTSEC-2026-0258 / GHSA-q83h-524g-xf6h**, a low-severity
unbounded empty HTTP/2 DATA-frame queue in `h2` 0.3.27 used by Actix. The
published fix is in h2 ≥0.4.16, not compatible with Actix's current h2 0.3
requirement; no unpublished dependency or advisory suppression is used. TLS
support enables Actix HTTP/2, so do not expose the agent directly to untrusted
peers: use the isolated HTTP/1.1 upstream described above until Actix adopts a
fixed dependency. This remains an upstream follow-up, not a clean Rust audit.

ServerWatch now owns its Actix routes instead of using `rusty-api`'s shared
query-password/GET route helper. No shared crate was changed or published;
Cricket/movie need their own coordinated upstream/client migration if they
use that helper.

## Contributing and license

Contributions are welcome. This project is licensed under the MIT License.
