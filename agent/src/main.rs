mod models;
mod getters;

use rusty_api::{Api, Routes, Cors, HttpResponse};
use actix_web::{web, http};
use std::{env, process::Command};

fn main() {
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
        .add_route_with_password("/reboot", reboot, password);

    Api::new()
        .certs("serverwatch.crt", "serverwatch.key")
        .rate_limit(1, 1)
        .bind("0.0.0.0", 49160)
        .configure_routes(routes)
        .configure_cors(|| {
            Cors::default()
                .allowed_methods(vec!["GET"])
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
    HttpResponse::Ok().body("Connection successful")
}

async fn shutdown() -> HttpResponse {
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