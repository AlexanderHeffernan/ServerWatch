# ServerWatch
> [!WARNING]
> The `ServerWatch-Agent` has only been tested on Raspberry Pi OS. It uses cross-platform Rust modules to gather system metrics, but compatibility on other systems is not guaranteed—use at your own risk.
## Overview
ServerWatch is a lightweight tool designed to monitor key server metrics like CPU, RAM, and disk usage, whether you're on-site or remote. Simply enter your server's IP address and password to access a clean, web-based dashboard. The tool consists of:
- **Frontend**: A static web dashboard displaying server metrics, hosted on GitHub Pages or deployable locally.
- **Agent Script**: A Rust-based background process running on your server, delivering system metrics securely over HTTPS.
- **Easy Setup**: Install and uninstall scripts (`install.sh` and `uninstall.sh`) streamline deployment and removal, with automatic background service configuration.
## Backstory
I created ServerWatch after setting up a Raspberry Pi 5 as a personal project server. I needed a simple way to check its performance remotely—CPU load, temperatures, etc.—and built this tool to fill that gap.
## Features
- Metrics: CPU usage, RAM usage, disk usage, CPU/component temperatures.
- Secure access via IP address and password over HTTPS.
- Efficient Rust-based agent with minimal resource usage.
- Static frontend hosted on GitHub Pages (`https://alexanderheffernan.github.io/ServerWatch/`).
- Open-source, with one-command install/uninstall scripts.

## Installation
### 1. Deploy the Frontend
The frontend is pre-deployed at [https://alexanderheffernan.github.io/ServerWatch/](https://alexanderheffernan.github.io/ServerWatch/). To run it locally:

```bash
# Ensure Node.js is installed
node -v  # Install if missing: sudo apt install nodejs npm

# Install http-server globally
npm install -g http-server

# Start the server from the frontend directory
cd /path/to/ServerWatch/frontend
http-server -p 8080
```
### 2. Install the Agent Script
Run the install.sh script on your server to set up the agent. It:
- Checks for prerequisites (curl, openssl).
- Downloads the pre-built ServerWatch-Agent binary (ARM only).
- Generates a self-signed certificate and key.
- Configures a systemd service to run the agent on boot.
**One Command Install**
```bash
curl -sSL https://raw.githubusercontent.com/AlexanderHeffernan/ServerWatch/main/install.sh | bash
```
- The agent will run at `https://<your-server-ip>:49160/metrics`.
### 3. Configure the Agent Password
The default password is Password123. To set a custom password:
- Edit the systemd service:
`sudo nano /etc/systemd/system/serverwatch-agent.service`
- Add under [Service]:
`Environment="PASSWORD=YourSecurePassword"`
- Apply changes:
```bash
sudo systemctl daemon-reload
sudo systemctl restart serverwatch-agent.service
```
## Usage
1. Open the frontend (GitHub Pages or local).
2. Enter your server’s IP address (e.g., 192.168.68.59) and password.
3. Click "Connect" to view your metrics.
- Local Access: Use your LAN IP (e.g., 192.168.68.59).
- Remote Access: Use your public IP (e.g., 161.29.239.150) with port forwarding set to 49160.

## Uninstall
To remove the agent:
`curl -sSL https://raw.githubusercontent.com/AlexanderHeffernan/ServerWatch/main/uninstall.sh | bash`
This stops the service, deletes files in ~/ServerWatch-agent/, and removes the systemd configuration.
