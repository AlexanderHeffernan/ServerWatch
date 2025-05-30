mod models;
mod getters;
mod push_notifications;
mod temperature_monitor;

use push_notifications::send_alertzy_notification;
use rusty_api::{Api, Routes, Cors, HttpResponse};
use actix_web::{web, http};
use std::{env, process::Command, collections::HashMap};
use temperature_monitor::TempConfig;

fn main() {
    // Spawn a background task to monitor temperature
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::spawn(async {
            temperature_monitor::monitor_temperature().await;
        });
    });
    
    // Check if the SERVERWATCH_PASSWORD environment variable is set
    let password: &'static str = Box::leak(
        env::var("SERVERWATCH_PASSWORD")
            .expect("SERVERWATCH_PASSWORD must be set.")    
            .into_boxed_str()
    );
    
    let routes = Routes::new()
        .add_route_with_password("/metrics", get_metrics, password)
        .add_route_with_password("/test-connection", test_connection, password)
        .add_route_with_password("/shutdown", shutdown, password)
        .add_route_with_password("/reboot", reboot, password)
        .add_route_with_password("/set-temp-config", set_temp_config, password)
        .add_route_with_password("/get-temp-config", get_temp_config, password);

    Api::new()
        .certs("serverwatch.crt", "serverwatch.key")
        .rate_limit(3, 10)
        .bind("0.0.0.0", 49160)
        .configure_routes(routes)
        .configure_cors(|| {
            Cors::default()
                .allowed_methods(vec!["GET", "POST", "PUT"])
                .allow_any_origin()
                .allowed_header(http::header::CONTENT_TYPE)
                .allowed_header("ngrok-skip-browser-warning")
        })
        .start();
}

async fn get_metrics() -> HttpResponse {
    use getters::*;
    use models::Metrics;

    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sys.refresh_all();

    HttpResponse::Ok().json(Metrics {
        total_cpu_usage: get_total_cpu_usage(&sys),
        individual_cpu_usage: get_individual_cpu_usage(&sys),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        cpu_temperature: get_temperature(),
        individual_temperatures: get_individual_temperatures(),
        disks: get_disks(),
    })
}

async fn test_connection() -> HttpResponse {
    use getters::get_server_name;

    HttpResponse::Ok().json(HashMap::from([("server_name", get_server_name())]))
}

pub async fn shutdown() -> HttpResponse {
    // Send a notification before shutting down
    if let Err(e) = send_alertzy_notification(
        "Server Shutdown Initiated",
        "The server is shutting down now. If you do you receive a follow-up error alert, then shutdown was successful.",
        2
    ).await { 
        eprintln!("Failed to send shutdown notification: {}", e); 
        return HttpResponse::InternalServerError().body(format!("Failed to send shutdown notification: {}", e));
    }

    // Run the shutdown command in a blocking context
    let result = web::block(|| {
        Command::new("sudo")
            .arg("/usr/sbin/shutdown")
            .arg("-h")
            .arg("now")
            .output()
    })
    .await;

    match result {
        Ok(Ok(output)) => {
            if output.status.success() {
                HttpResponse::Ok().body("Shutting down now...")
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                HttpResponse::InternalServerError().body(format!("Failed to shut down: {}", error))
            }
        }
        Ok(Err(e)) => HttpResponse::InternalServerError().body(format!("Error executing shutdown command: {}", e)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Blocking task failed: {}", e)),
    }
}

async fn reboot() -> HttpResponse {
    // Send a notification before rebooting
    if let Err(e) = send_alertzy_notification(
        "Server Reboot Initiated",
        "The server is rebooting now. If you do you receive a follow-up error alert, then reboot was successful.",
        2
    ).await { 
        eprintln!("Failed to send reboot notification: {}", e); 
        return HttpResponse::InternalServerError().body(format!("Failed to send reboot notification: {}", e));
    }

    // Run the reboot command in a blocking context
    let result = web::block(|| {
        Command::new("sudo")
            .arg("/usr/sbin/reboot")
            .output()
    })
    .await;

    match result {
        Ok(Ok(output)) => {
            if output.status.success() {
                HttpResponse::Ok().body("Rebooting now...")
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                HttpResponse::InternalServerError().body(format!("Failed to reboot: {}", error))
            }
        }
        Ok(Err(e)) => HttpResponse::InternalServerError().body(format!("Error executing reboot command: {}", e)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Blocking task failed: {}", e)),
    }
}

async fn set_temp_config(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    // Extract query parameters
    let warning_temp = query.get("warning_temp").and_then(|v| v.parse::<u64>().ok());
    let shutdown_temp = query.get("shutdown_temp").and_then(|v| v.parse::<u64>().ok());
    let warnings_enabled = query.get("warnings_enabled").and_then(|v| v.parse::<bool>().ok());
    let shutdown_enabled = query.get("shutdown_enabled").and_then(|v| v.parse::<bool>().ok());

    // Validate that all required parameters are present
    if let (Some(warning_temp), Some(shutdown_temp), Some(warnings_enabled), Some(shutdown_enabled)) =
        (warning_temp, shutdown_temp, warnings_enabled, shutdown_enabled)
    {
        // Log the received configuration
        println!(
            "Received new config: warning_temp={}, shutdown_temp={}, warnings_enabled={}, shutdown_enabled={}",
            warning_temp, shutdown_temp, warnings_enabled, shutdown_enabled
        );

        // Save the configuration to the file
        let new_config = TempConfig {
            warning_temp,
            shutdown_temp,
            warnings_enabled,
            shutdown_enabled,
        };
        new_config.save_to_file("config.json");

        HttpResponse::Ok().body("Temperature settings updated successfully")
    } else {
        HttpResponse::BadRequest().body("Missing or invalid query parameters")
    }
}

async fn get_temp_config() -> HttpResponse {
    // Load the configuration from the file
    let config = TempConfig::load_from_file("config.json");

    // Return the configuration as JSON
    HttpResponse::Ok().json(config)
}