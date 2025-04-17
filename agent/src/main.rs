mod models;
mod getters;

use rusty_api::{Api, Routes, Cors, HttpResponse};
use actix_web::http;
use std::env;

fn main() {
    // Check if the SERVERWATCH_PASSWORD environment variable is set
    let password: &'static str = Box::leak(
        env::var("SERVERWATCH_PASSWORD")
            .expect("SERVERWATCH_PASSWORD must be set.")    
            .into_boxed_str()
    );
    
    let routes = Routes::new()
        .add_route_with_password("/metrics", get_metrics, password);

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

    rusty_api::HttpResponse::Ok().json(Metrics {
        total_cpu_usage: get_total_cpu_usage(&sys),
        individual_cpu_usage: get_individual_cpu_usage(&sys),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        cpu_temperature: get_temperature(),
        individual_temperatures: get_individual_temperatures(),
        disks: get_disks(),
    })
}