use axum::{routing::get, Router, Json, http::Method, extract::Query, http::StatusCode};
use tower_http::cors::{CorsLayer, AllowOrigin};
use serde::{Serialize, Deserialize};
use sysinfo::{Disks, System, Components};
use std::net::SocketAddr;
use std::env;
use axum_server::tls_rustls::RustlsConfig;

#[derive(Serialize)]
struct TemperatureComponent {
    label: String,
    temperature: u64,
    max: u64,
    critical: u64,
}

#[derive(Serialize)]
struct Disk {
    label: String,
    total: u64,
    used: u64,
}

#[derive(Serialize)]
struct Metrics {
    total_cpu_usage: f32,
    individual_cpu_usage: Vec<f32>,
    memory_usage: u64,
    memory_total: u64,
    cpu_temperature: u64,
    individual_temperatures: Vec<TemperatureComponent>,
    disks: Vec<Disk>,
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
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sys.refresh_all();

    Ok(Json(Metrics {
        total_cpu_usage: get_total_cpu_usage(&sys),
        individual_cpu_usage: get_individual_cpu_usage(&sys),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        cpu_temperature: get_temperature(),
        individual_temperatures: get_individual_temperatures(),
        disks: get_disks(),
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

fn get_temperature() -> u64 {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        if component.label().contains("cpu") {
            return component.temperature().unwrap_or(0.0) as u64;
        }
    }
    if components.is_empty() {
        return 0;
    }
    components[0].temperature().unwrap_or(0.0) as u64
}

fn get_individual_temperatures() -> Vec<TemperatureComponent> {
    let components = Components::new_with_refreshed_list();
    components
        .iter()
        .map(|component| TemperatureComponent {
            label: component.label().to_string(),
            temperature: component.temperature().unwrap_or(0.0) as u64,
            max: component.max().unwrap_or(0.0) as u64,
            critical: component.critical().unwrap_or(0.0) as u64,
        })
        .collect()
}

fn get_disks() -> Vec<Disk> {
    Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Disk {
            label: disk.file_system().to_string_lossy().into_owned(),
            total: disk.total_space(),
            used: disk.total_space() - disk.available_space(),
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET])
                .allow_origin(AllowOrigin::exact(
                        "https://alexanderheffernan.github.io/ServerWatch".parse().unwrap(),
                ))
                .allow_headers(vec![axum::http::header::CONTENT_TYPE, axum::http::header::HeaderName::from_static("ngrok-skip-browser-warning")]) // Add custom header
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 49160));

    // Load TLS certificate and key from files
    let tls_config = RustlsConfig::from_pem_file(
        "serverwatch.crt",
        "serverwatch.key",
    )
    .await?;

    println!("Agent running at https://{}", addr);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
