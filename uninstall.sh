#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting ServerWatch Agent uninstallation...${NC}"

# Define install directory and service
INSTALL_DIR="$HOME/ServerWatch-agent"
SERVICE_NAME="serverwatch-agent.service"
SERVICE_FILE="/etc/systemd/system/$SERVICE_NAME"

# Stop and disable the service
if systemctl is-active "$SERVICE_NAME" >/dev/null 2>&1; then
    echo "Stopping $SERVICE_NAME..."
    sudo systemctl stop "$SERVICE_NAME"
fi

if systemctl is-enabled "$SERVICE_NAME" >/dev/null 2>&1; then
    echo "Disabling $SERVICE_NAME..."
    sudo systemctl disable "$SERVICE_NAME"
fi

# Remove the service file
if [ -f "$SERVICE_FILE" ]; then
    echo "Removing service file $SERVICE_FILE..."
    sudo rm "$SERVICE_FILE"
fi

# Reload systemd
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Remove the install directory
if [ -d /opt/serverwatch ]; then
    sudo rm -rf /opt/serverwatch /etc/serverwatch /var/lib/serverwatch
    sudo rm -f /etc/sudoers.d/serverwatch-power
    # Keep the locked service account so existing file ownership cannot be reassigned.
fi
if [ -d "$INSTALL_DIR" ]; then
    echo "Removing installation directory $INSTALL_DIR..."
    rm -rf "$INSTALL_DIR"
else
    echo "No installation directory found at $INSTALL_DIR."
fi

# Verify removal
if systemctl list-units --all | grep -q "$SERVICE_NAME"; then
    echo -e "${RED}Warning: $SERVICE_NAME still appears in systemd. Check 'systemctl list-units --all'.${NC}"
else
    echo -e "${GREEN}Service $SERVICE_NAME successfully removed from systemd.${NC}"
fi

echo -e "${GREEN}Uninstallation complete!${NC}"
