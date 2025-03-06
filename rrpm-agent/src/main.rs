use axum::{routing::get,Router,Json,http::Method, extract::Query, http::StatusCode};
use tower_http::cors::{CorsLayer, Any};
use serde::{Serialize, Deserialize};
use sysinfo::{Disks, System};
use std::net::SocketAddr;
use std::env;

#[derive(Serialize)]
struct Metrics {
    cpu_usage: Vec<f32>, // CPU usage percentage
    memory_usage: u64, // Memory usage in bytes (raw data)
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

    let cpu_usage = get_cpu_usage(&sys); // CPU usage in percentage

    let memory_usage = sys.used_memory(); // Memory usage in bytes

    let disk_usage = get_disk_usage(); // Disk usage in bytes
                                                         
    Ok(Json(Metrics {
        cpu_usage,
        memory_usage,
        disk_usage,
    }))
}

fn get_cpu_usage(sys: &System) -> Vec<f32> {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect() // Collect CPU usage values into a vector
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
