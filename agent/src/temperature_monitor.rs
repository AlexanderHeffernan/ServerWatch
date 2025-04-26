use tokio::time::{self, Duration};
use crate::getters::get_temperature;
use crate::push_notifications::send_alertzy_notification;
use crate::shutdown;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct TempConfig {
    pub warning_temp: u64,
    pub shutdown_temp: u64,
    pub warnings_enabled: bool,
    pub shutdown_enabled: bool,
}

impl TempConfig {
    pub fn load_from_file(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(data) => {
                serde_json::from_str(&data).expect("Failed to parse config file")
            }
            Err(_) => {
                // If the file doesn't exist or can't be read, create it with default values
                eprintln!("Config file not found. Creating a new one with default values.");
                let default_config = TempConfig {
                    warning_temp: 75,
                    shutdown_temp: 85,
                    warnings_enabled: true,
                    shutdown_enabled: true,
                };
                default_config.save_to_file(path);
                default_config
            }
        }
    }

    pub fn save_to_file(&self, path: &str) {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(path, data).expect("Failed to write config file");
    }
}

pub async fn monitor_temperature() {
    let mut interval = time::interval(Duration::from_secs(30));
    let mut warning_sent = false;

    loop {
        interval.tick().await;

        let config = TempConfig::load_from_file("config.json");

        println!("Warning Temperature: {}°C", config.warning_temp);
        println!("Shutdown Temperature: {}°C", config.shutdown_temp);
        println!("Warnings Enabled: {}", config.warnings_enabled);
        println!("Shutdown Enabled: {}", config.shutdown_enabled);

        let temperature = get_temperature();
        if config.shutdown_enabled && temperature >= config.shutdown_temp {
            handle_shutdown(temperature).await;
            break; // Exit the loop after shutdown
        } else if config.warnings_enabled && temperature >= config.warning_temp {
            if !warning_sent {
                handle_warning(temperature).await;
                warning_sent = true; // Prevent sending multiple warnings
            }
        } else {
            warning_sent = false;
        }
    }
}

async fn handle_shutdown(temperature: u64) {
    eprintln!("Critical temperature reached: {}°C. Shutting down...", temperature);
    if let Err(e) = send_alertzy_notification(
        "Critical Temperature Reached",
        &format!("The system temperature reached {}°C. Shutting down to prevent damage.", temperature),
        1
    ).await {
        eprintln!("Failed to send shutdown notification: {}", e);
    }
    
    shutdown().await;
}

async fn handle_warning(temperature: u64) {
    if let Err(e) = send_alertzy_notification(
        "High Temperature Warning",
        &format!("The system temperature is at {}°C. Please check the system.", temperature),
        1
    ).await {
        eprintln!("Failed to send warning notification: {}", e);
    }
}