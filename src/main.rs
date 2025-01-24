// src/main.rs
use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode,
};
use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize metrics
    let _ = metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 9091))
        .build();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check));

    // Run our application
    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));
    tracing::info!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
