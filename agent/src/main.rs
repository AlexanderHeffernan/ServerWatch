use axum::{routing::get,Router,Json,http::Method, extract::Query, http::StatusCode};
use tower_http::cors::{CorsLayer, Any};
use serde::{Serialize, Deserialize};
use sysinfo::{Disks, System};
use std::net::SocketAddr;
use std::env;

#[derive(Serialize)]
struct Metrics {
    total_cpu_usage: f32, // CPU usage percentage averages across all cores
    individual_cpu_usage: Vec<f32>, // CPU usage percentage for each core
    memory_usage: u64, // Memory usage in bytes (raw data)
    memory_total: u64, // Total memory in bytes (raw data)
    disk_usage: Vec<u64>, // Disk usage in bytes (raw data)
}

#[derive(Deserialize)]
struct AuthQuery {
    password: String,
}

async fn get_metrics(query: Query<AuthQuery>) -> Result<Json<Metrics>, StatusCode> {
    let expected_password = env::var("PASSWORD").unwrap_or_else(|_| "Password123".to_string());

    if query.password != expected_password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(1000)); // Wait for a second
    sys.refresh_all(); // Second refresh to get actual usage

    Ok(Json(Metrics {
        total_cpu_usage: get_total_cpu_usage(&sys),
        individual_cpu_usage: get_individual_cpu_usage(&sys),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        disk_usage: get_disk_usage(),
    }))
}

fn get_total_cpu_usage(sys: &System) -> f32 {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32
}

fn get_individual_cpu_usage(sys: &System) -> Vec<f32> {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect()
}

fn get_disk_usage() -> Vec<u64> {
    Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| disk.total_space() - disk.available_space())
        .collect()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET]) // Allow GET requests
                .allow_origin(Any), // Allow requests from any origin
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Agent running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
