use crate::getters::get_temperature;
use crate::push_notifications::send_alertzy_notification;
use crate::shutdown;
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::time::{self, Duration};

#[derive(Serialize, Deserialize)]
pub struct TempConfig {
    pub warning_temp: u64,
    pub shutdown_temp: u64,
    pub warnings_enabled: bool,
    pub shutdown_enabled: bool,
}

impl TempConfig {
    pub fn valid(&self) -> bool {
        (40..=150).contains(&self.warning_temp)
            && (40..=150).contains(&self.shutdown_temp)
            && self.warning_temp < self.shutdown_temp
    }

    pub fn load_from_file(path: &str) -> Self {
        match fs::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str::<Self>(&data).ok())
            .filter(Self::valid)
        {
            Some(config) => config,
            None => {
                eprintln!("Missing or invalid temperature config; using safe defaults.");
                TempConfig {
                    warning_temp: 75,
                    shutdown_temp: 85,
                    warnings_enabled: true,
                    shutdown_enabled: true,
                }
            }
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        use std::io::Write;
        let data = serde_json::to_string_pretty(self)?;
        let parent = std::path::Path::new(path)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(data.as_bytes())?;
        temporary.as_file().sync_all()?;
        temporary.persist(path).map_err(|e| e.error)?;
        Ok(())
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
            // Retry on the next tick if the host did not actually shut down.
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
    eprintln!(
        "Critical temperature reached: {}°C. Shutting down...",
        temperature
    );
    shutdown().await;
}

async fn handle_warning(temperature: u64) {
    if let Err(e) = send_alertzy_notification(
        "High Temperature Warning",
        &format!(
            "The system temperature is at {}°C. Please check the system.",
            temperature
        ),
        1,
    )
    .await
    {
        eprintln!("Failed to send warning notification: {}", e);
    }
}
