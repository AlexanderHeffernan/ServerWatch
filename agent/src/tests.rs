use super::*;
use actix_web::{
    http::{Method, StatusCode},
    test,
};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};

#[actix_web::test]
async fn api_auth_methods_and_configuration() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.json");
    // Never install the real power executor in a test app.
    fn fake_power(_: bool) -> io::Result<std::process::Output> {
        Command::new("/bin/true").output()
    }
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(Credentials {
                admin: "admin &?#".into(),
                read: Some("reader".into()),
            }))
            .app_data(web::Data::new(ConfigPath(path.to_str().unwrap().into())))
            .app_data(web::Data::new(PowerExecutor(fake_power)))
            .app_data(web::Data::new(RwLock::new(Some(sample_metrics()))))
            .configure(routes),
    )
    .await;
    let endpoints = [
        (Method::GET, "/test-connection"),
        (Method::GET, "/metrics"),
        (Method::GET, "/get-temp-config"),
        (Method::PUT, "/set-temp-config"),
        (Method::POST, "/shutdown"),
        (Method::POST, "/reboot"),
    ];
    let config = json!({"warning_temp": 61, "shutdown_temp": 92, "warnings_enabled": false, "shutdown_enabled": true});
    for (method, uri) in &endpoints {
        for credential in [None, Some("Bearer wrong"), Some("Basic reader")] {
            let mut request = test::TestRequest::default()
                .method(method.clone())
                .uri(uri)
                .set_json(&config);
            if let Some(value) = credential {
                request = request.insert_header((header::AUTHORIZATION, value));
            }
            assert_eq!(
                test::call_service(&app, request.to_request())
                    .await
                    .status(),
                StatusCode::UNAUTHORIZED,
                "{method} {uri}"
            );
        }
        let query = test::TestRequest::default()
            .method(method.clone())
            .uri(&format!("{uri}?password=admin%20%26%3F%23"))
            .set_json(&config)
            .to_request();
        assert_eq!(
            test::call_service(&app, query).await.status(),
            StatusCode::UNAUTHORIZED
        );
        for (credential, expected) in [
            (
                "reader",
                if *method == Method::GET {
                    StatusCode::OK
                } else {
                    StatusCode::FORBIDDEN
                },
            ),
            ("admin &?#", StatusCode::OK),
        ] {
            let request = test::TestRequest::default()
                .method(method.clone())
                .uri(uri)
                .insert_header((header::AUTHORIZATION, format!("Bearer {credential}")))
                .set_json(&config)
                .to_request();
            assert_eq!(
                test::call_service(&app, request).await.status(),
                expected,
                "{credential}: {method} {uri}"
            );
        }
    }
    for uri in ["/shutdown", "/reboot", "/set-temp-config"] {
        let request = test::TestRequest::get()
            .uri(uri)
            .insert_header((header::AUTHORIZATION, "Bearer admin &?#"))
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::METHOD_NOT_ALLOWED
        );
    }
    for (warning, shutdown) in [(39, 90), (75, 151), (90, 90), (91, 90), (0, 0)] {
        let invalid = json!({"warning_temp":warning, "shutdown_temp":shutdown, "warnings_enabled":true,"shutdown_enabled":true});
        let request = test::TestRequest::put()
            .uri("/set-temp-config")
            .insert_header((header::AUTHORIZATION, "Bearer admin &?#"))
            .set_json(&invalid)
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::BAD_REQUEST
        );
    }
    let request = test::TestRequest::get()
        .uri("/get-temp-config")
        .insert_header((header::AUTHORIZATION, "Bearer reader"))
        .to_request();
    let saved: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(saved, config); // Denied/invalid writes must not change persisted state.
    std::fs::write(&path, "broken JSON").unwrap();
    let request = test::TestRequest::get()
        .uri("/get-temp-config")
        .insert_header((header::AUTHORIZATION, "Bearer reader"))
        .to_request();
    let fallback: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(fallback["shutdown_temp"], 85);
    assert_eq!(fallback["shutdown_enabled"], true);

    // Both inclusive limits are valid, and a failed persist must be reported.
    let limits = json!({"warning_temp":40,"shutdown_temp":150,"warnings_enabled":true,"shutdown_enabled":false});
    let request = test::TestRequest::put()
        .uri("/set-temp-config")
        .insert_header((header::AUTHORIZATION, "Bearer admin &?#"))
        .set_json(&limits)
        .to_request();
    assert_eq!(
        test::call_service(&app, request).await.status(),
        StatusCode::OK
    );
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    let request = test::TestRequest::put()
        .uri("/set-temp-config")
        .insert_header((header::AUTHORIZATION, "Bearer admin &?#"))
        .set_json(&config)
        .to_request();
    assert_eq!(
        test::call_service(&app, request).await.status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[actix_web::test]
async fn cors_allows_only_configured_origin_and_headers() {
    let app = test::init_service(
        App::new()
            .wrap(cors(&["https://dashboard.example".into()]))
            .configure(routes),
    )
    .await;
    for (origin, expected) in [
        ("https://dashboard.example", StatusCode::OK),
        ("https://evil.example", StatusCode::BAD_REQUEST),
        ("null", StatusCode::BAD_REQUEST),
    ] {
        let request = test::TestRequest::default()
            .method(Method::OPTIONS)
            .uri("/set-temp-config")
            .insert_header((header::ORIGIN, origin))
            .insert_header((header::ACCESS_CONTROL_REQUEST_METHOD, "PUT"))
            .insert_header((
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "authorization,content-type,ngrok-skip-browser-warning",
            ))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), expected);
        if expected == StatusCode::OK {
            assert_eq!(
                response
                    .headers()
                    .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                    .unwrap(),
                origin
            );
        } else {
            assert!(
                response
                    .headers()
                    .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                    .is_none()
            );
        }
    }
}

#[actix_web::test]
async fn failed_or_hanging_notifications_never_block_power() {
    let calls = AtomicUsize::new(0);
    for result in [Ok(()), Err("Missing key or HTTP/JSON failure".into())] {
        let response = notify_then_power(ready(result), || async {
            calls.fetch_add(1, Ordering::SeqCst);
            HttpResponse::Accepted().finish()
        })
        .await;
        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
    let start = std::time::Instant::now();
    let response = notify_then_power(std::future::pending(), || async {
        calls.fetch_add(1, Ordering::SeqCst);
        HttpResponse::Accepted().finish()
    })
    .await;
    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert!(start.elapsed() < Duration::from_secs(3));
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[actix_web::test]
async fn peer_rate_limit_rejects_excess_requests() {
    let governor = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(10)
        .finish()
        .unwrap();
    let app = test::init_service(
        App::new()
            .wrap(Governor::new(&governor))
            .app_data(web::Data::new(Credentials {
                admin: "admin".into(),
                read: None,
            }))
            .configure(routes),
    )
    .await;
    for index in 0..11 {
        let request = test::TestRequest::get()
            .uri("/test-connection")
            .peer_addr("127.0.0.1:12345".parse().unwrap())
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(
            response.status(),
            if index < 10 {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::TOO_MANY_REQUESTS
            }
        );
    }
}
