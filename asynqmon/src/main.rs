//! Asynq Members - Task Control Panel
//!
//! A task control panel UI built with Dioxus, inspired by @hibiken/asynqmon.
//! This version provides a web interface with actix-web backend.

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_cors::Cors;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    tracing::info!("🚀 Web interface available at http://127.0.0.1:8080");
    tracing::info!("📊 Open http://127.0.0.1:8080 in your browser");

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index_handler))
            .route("/api/queues", web::get().to(api::get_queues))
            .route("/api/queue/{name}", web::get().to(api::get_queue_info))
            .route("/api/servers", web::get().to(api::get_servers))
            .route("/api/tasks/{queue}/{state}", web::get().to(api::get_tasks))
            .route("/api/connect", web::post().to(api::connect))
            .route("/api/pause/{queue}", web::post().to(api::pause_queue))
            .route("/api/unpause/{queue}", web::post().to(api::unpause_queue))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Index handler that serves the Dioxus app
async fn index_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}
