//! Asynq Members - Task Control Panel
//!
//! A task control panel UI built with Dioxus, inspired by @hibiken/asynqmon.
//! This version provides a web interface.

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing_subscriber;

mod api;
mod inspector_service;

use inspector_service::InspectorService;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub inspector: Arc<InspectorService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Asynq Members - Task Control Panel");

    // Create inspector service
    let inspector = Arc::new(InspectorService::new());

    // Try to connect to default Redis instance
    match inspector
        .initialize("redis://127.0.0.1:6379".to_string())
        .await
    {
        Ok(_) => tracing::info!("Connected to Redis at default location"),
        Err(e) => tracing::warn!("Could not connect to default Redis: {}", e),
    }

    // Create app state
    let state = AppState { inspector };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/queues", get(api::get_queues))
        .route("/api/queue/{name}", get(api::get_queue_info))
        .route("/api/servers", get(api::get_servers))
        .route("/api/tasks/{queue}/{state}", get(api::get_tasks))
        .route("/api/connect", axum::routing::post(api::connect))
        .route("/api/pause/{queue}", axum::routing::post(api::pause_queue))
        .route(
            "/api/unpause/{queue}",
            axum::routing::post(api::unpause_queue),
        )
        .with_state(state);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("🚀 Web interface available at http://{}", addr);
    tracing::info!("📊 Open http://127.0.0.1:8080 in your browser");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Index handler that serves the HTML page
async fn index_handler() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}
