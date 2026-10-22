mod getters;
mod models;
mod push_notifications;
mod temperature_monitor;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    App, FromRequest, HttpRequest, HttpResponse, HttpServer, dev::Payload, http::header, web,
};
use std::{
    collections::HashMap,
    env,
    future::{Ready, ready},
    io,
    process::Command,
    sync::RwLock,
    time::Duration,
};
use subtle::ConstantTimeEq;
use temperature_monitor::TempConfig;

struct Credentials {
    admin: String,
    read: Option<String>,
}

// Explicit header authentication only: never accept cookies or URL credentials.
struct Auth;
impl FromRequest for Auth {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let credentials = req.app_data::<web::Data<Credentials>>().unwrap();
        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));
        let matches = |secret: &str| {
            token.is_some_and(|token| bool::from(token.as_bytes().ct_eq(secret.as_bytes())))
        };
        let admin = matches(&credentials.admin);
        let read = credentials.read.as_deref().is_some_and(matches);
        let mutation = req.method() != actix_web::http::Method::GET
            && req.method() != actix_web::http::Method::HEAD;
        ready(if admin || (read && !mutation) {
            Ok(Auth)
        } else if read {
            Err(actix_web::error::ErrorForbidden(
                "Administrator credential required",
            ))
        } else {
            Err(actix_web::error::ErrorUnauthorized(
                "Bearer credential required",
            ))
        })
    }
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/metrics").route(web::get().to(get_metrics)))
        .service(web::resource("/test-connection").route(web::get().to(test_connection)))
        .service(web::resource("/shutdown").route(web::post().to(shutdown_route)))
        .service(web::resource("/reboot").route(web::post().to(reboot)))
        .service(web::resource("/set-temp-config").route(web::put().to(set_temp_config)))
        .service(web::resource("/get-temp-config").route(web::get().to(get_temp_config)));
}

fn cors(origins: &[String]) -> Cors {
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("ngrok-skip-browser-warning"),
        ]);
    for origin in origins {
        cors = cors.allowed_origin(origin);
    }
    cors
}

type MetricsCache = RwLock<Option<models::Metrics>>;
struct PowerExecutor(fn(bool) -> io::Result<std::process::Output>);
struct ConfigPath(String);

#[actix_web::main]
async fn main() -> io::Result<()> {
    let admin = env::var("SERVERWATCH_PASSWORD")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .expect("Nonempty SERVERWATCH_PASSWORD must be set");
    let read = env::var("SERVERWATCH_READ_PASSWORD").ok();
    assert!(
        read.as_ref()
            .is_none_or(|v| !v.trim().is_empty() && v != &admin),
        "Read password must be nonempty and different from admin password"
    );
    let credentials = web::Data::new(Credentials { admin, read });
    let origins: Vec<String> = env::var("SERVERWATCH_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "https://serverwatch.alexheffernan.dev".into())
        .split(',')
        .map(|v| v.trim().to_owned())
        .filter(|v| !v.is_empty())
        .collect();
    for origin in &origins {
        let url = reqwest::Url::parse(origin).expect("Invalid allowed origin");
        assert!(
            matches!(url.scheme(), "https" | "http")
                && url.origin().ascii_serialization() == *origin,
            "Configure exact origins without paths or trailing slashes"
        );
    }
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
    let certs = CertificateDer::pem_file_iter("serverwatch.crt")
        .map_err(io::Error::other)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(io::Error::other)?;
    let key = PrivateKeyDer::from_pem_file("serverwatch.key").map_err(io::Error::other)?;
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    let tls = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(io::Error::other)?;
    let cache = web::Data::new(RwLock::new(None));
    let sampler = cache.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(metrics) = web::block(sample_metrics).await {
                *sampler.write().unwrap() = Some(metrics);
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
    tokio::spawn(temperature_monitor::monitor_temperature());
    let bind = env::var("SERVERWATCH_BIND").unwrap_or_else(|_| "127.0.0.1:49160".into());
    let governor = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(10)
        .finish()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(credentials.clone())
            .app_data(cache.clone())
            .app_data(web::Data::new(PowerExecutor(execute_power)))
            .app_data(web::Data::new(ConfigPath("config.json".into())))
            .app_data(web::JsonConfig::default().limit(1024))
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add((header::CACHE_CONTROL, "no-store")),
            )
            .wrap(Governor::new(&governor))
            .wrap(cors(&origins))
            .configure(routes)
    })
    .bind_rustls_0_23(bind, tls)?
    .run()
    .await
}

fn sample_metrics() -> models::Metrics {
    use getters::*;
    let mut sys = sysinfo::System::new_all();
    std::thread::sleep(Duration::from_secs(1));
    sys.refresh_all();
    models::Metrics {
        total_cpu_usage: get_total_cpu_usage(&sys),
        individual_cpu_usage: get_individual_cpu_usage(&sys),
        memory_usage: sys.used_memory(),
        memory_total: sys.total_memory(),
        cpu_temperature: get_temperature(),
        individual_temperatures: get_individual_temperatures(),
        disks: get_disks(),
    }
}

async fn get_metrics(_: Auth, cache: web::Data<MetricsCache>) -> HttpResponse {
    match cache.read().unwrap().as_ref() {
        Some(metrics) => HttpResponse::Ok().json(metrics),
        None => HttpResponse::ServiceUnavailable().body("Metrics warming up; retry shortly"),
    }
}

async fn test_connection(_: Auth) -> HttpResponse {
    HttpResponse::Ok().json(HashMap::from([("server_name", getters::get_server_name())]))
}

// Power execution is passed separately so failure-path tests cannot invoke host commands.
async fn notify_then_power<N, P, F>(notification: N, power: P) -> HttpResponse
where
    N: std::future::Future<Output = Result<(), String>>,
    P: FnOnce() -> F,
    F: std::future::Future<Output = HttpResponse>,
{
    match tokio::time::timeout(Duration::from_secs(2), notification).await {
        Ok(Ok(())) => {}
        _ => eprintln!("Power notification failed or timed out; proceeding with power action"),
    }
    power().await
}

fn execute_power(reboot: bool) -> io::Result<std::process::Output> {
    let mut command = Command::new("/usr/bin/sudo");
    command.arg("-n");
    if reboot {
        command.arg("/usr/sbin/reboot");
    } else {
        command.args(["/usr/sbin/shutdown", "-h", "now"]);
    }
    command.output()
}

async fn power(
    reboot: bool,
    executor: fn(bool) -> io::Result<std::process::Output>,
) -> HttpResponse {
    let result = web::block(move || executor(reboot)).await;
    match result {
        Ok(Ok(output)) if output.status.success() => {
            HttpResponse::Ok().body("Power action initiated")
        }
        _ => {
            eprintln!("Power command failed; check service permissions");
            HttpResponse::InternalServerError().body("Power action failed")
        }
    }
}

pub async fn shutdown() -> HttpResponse {
    shutdown_with(execute_power).await
}
async fn shutdown_with(executor: fn(bool) -> io::Result<std::process::Output>) -> HttpResponse {
    notify_then_power(
        push_notifications::send_alertzy_notification(
            "Server Shutdown Initiated",
            "The server is shutting down now.",
            2,
        ),
        || power(false, executor),
    )
    .await
}
async fn shutdown_route(_: Auth, executor: web::Data<PowerExecutor>) -> HttpResponse {
    shutdown_with(executor.0).await
}
async fn reboot(_: Auth, executor: web::Data<PowerExecutor>) -> HttpResponse {
    notify_then_power(
        push_notifications::send_alertzy_notification(
            "Server Reboot Initiated",
            "The server is rebooting now.",
            2,
        ),
        || power(true, executor.0),
    )
    .await
}

async fn set_temp_config(
    _: Auth,
    config: web::Json<TempConfig>,
    path: web::Data<ConfigPath>,
) -> HttpResponse {
    if !config.valid() {
        return HttpResponse::BadRequest()
            .body("Temperatures must be 40–150°C and warning must be below shutdown");
    }
    match config.save_to_file(&path.0) {
        Ok(()) => HttpResponse::Ok().body("Temperature settings updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Could not save temperature settings"),
    }
}
async fn get_temp_config(_: Auth, path: web::Data<ConfigPath>) -> HttpResponse {
    HttpResponse::Ok().json(TempConfig::load_from_file(&path.0))
}

#[cfg(test)]
mod tests;
