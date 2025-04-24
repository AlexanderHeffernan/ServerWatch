use tokio::time::{self, Duration};
use crate::getters::get_temperature;
use crate::push_notifications::send_alertzy_notification;
use crate::shutdown;

const WARNING_TEMPERATURE: u64 = 75;
const SHUTDOWN_TEMPERATURE: u64 = 85;

pub async fn monitor_temperature() {
    let mut interval = time::interval(Duration::from_secs(30));
    let mut warning_sent = false;

    loop {
        interval.tick().await;

        let temperature = get_temperature();
        if temperature >= SHUTDOWN_TEMPERATURE {
            handle_shutdown(temperature).await;
            break; // Exit the loop after shutdown
        } else if temperature >= WARNING_TEMPERATURE {
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