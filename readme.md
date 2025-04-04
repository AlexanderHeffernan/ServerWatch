# ServerWatch
## Overview
ServerWatch is a **server monitoring tool** designed to display key server metrics, such as CPU and RAM usage. The tool consists of:
- **Frontend**: A static web page that displays server analytics.
- **Agent Script**: A Rust-based background process running on the server to collect data.
- **Connection**: The frontend connects to remote servers via IP address and password to fetch system metrics.
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
