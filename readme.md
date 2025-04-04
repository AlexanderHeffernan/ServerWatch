# ServerWatch
## Overview
ServerWatch is a simple tool that makes it easy to view your key server metrics, such as CPU and RAM usage. Whether you are on-site or remote, easily check out your servers metrics by simply entering your servers IP address and a password. The tool consists of:
- **Frontend**: A static web-based dashboard that displays key server metrics, accessible either through this repositories GitHub Pages deployment, or via your own personal local setup.
- **Agent Script**: A Rust-based background process which runs on the server. When fetched for, it will return the system metrics to the frontend for display.
- **Connection**: The frontend connects the remote server via an IP address, with a password setup on the server for authorised access.
- **Easy Setup**: Using the install.sh and uninstall.sh scripts, you can easily add and remove the agent script from your server. The install.sh script will ensure the agent is run in the background on start-up. The uninstall.sh script will remove all files and background processes.
## Backstory
ServerWatch was originally built for my **Raspberry Pi** to monitor my server's performance remotely. Since the Raspberry Pi is the only server-type device I have tested it on, it is the only confirmed supported device at this time. However, it may work on other systems as well, as the agent script uses a cross-platform module to collect the metrics.
## Features
- CPU and RAM usage tracking.
- Secure connection via IP and password.
- Rust-based efficient agent script.
- Simple, static frontend deployed via GitHub Pages.
- Open-source and easy to use.

## Installation
### 1. Deploy the Frontend
The frontend is already deployed via GitHub Pages at https://alexanderheffernan.github.io/ServerWatch/. You are also able to run it locally via a simple http web server.
### 2. Set Up the Agent Script
To install and run the Rust-based agent on your server:
```
git clone https://github.com/alexanderheffernan/ServerWatch.git
cd your-repo/agent
cargo build --release
./target/release/rrpm-agent
```
This will start the monitoring service on the server.
## Usage
1. Open the frontend in a browser.
2. Enter the **IP address** and **password** of a server running the agent.
3. View your metrics.
## Security Considerations
- Do not expose the agent script to the internet without a password setup.
- Store credentials securely using environment variables instead of hardcoding.
## Contributing
Contributions are welcome! Feel free to submit issues and pull requests.
## License
This project is licensed under the MIT License.
