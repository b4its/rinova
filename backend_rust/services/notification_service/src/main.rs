//! Notification Service
//!
//! Microservice for real-time notifications and email notifications.
//!
//! ## Features
//!
//! - Real-time notifications via WebSocket
//! - Offline notification buffering (up to 1000 per user)
//! - Email notifications for important events
//! - Notification history (30-day retention, 10k max)
//! - ZeroMQ event subscription
//!
//! ## Endpoints
//!
//! - GET    /api/v1/notifications          - List notifications
//! - POST   /api/v1/notifications/mark-read - Mark as read
//! - GET    /api/v1/notifications/unread-count - Unread count
//! - GET    /ws                            - WebSocket endpoint

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod services;

use handlers::*;
use services::{EmailService, NotificationService, NotificationStore, WebSocketManager};

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    online_users: usize,
}

/// Health check endpoint
async fn health(ws_manager: web::Data<WebSocketManager>) -> web::Json<HealthResponse> {
    let online_users = ws_manager.online_user_count().await;

    web::Json(HealthResponse {
        status: "healthy".to_string(),
        service: "notification_service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        online_users,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    // Get configuration
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "rinova".to_string());
    let port = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8006".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    tracing::info!("Starting Notification Service on port {}", port);

    // Initialize services
    let store = NotificationStore::new(&mongodb_uri, &mongodb_database)
        .await
        .expect("Failed to create notification store");

    let email_config = services::EmailConfig::from_env()
        .expect("Failed to load email configuration");
    let email_service = EmailService::new(email_config);

    let ws_manager = WebSocketManager::new();

    let notification_service = NotificationService::new(
        store,
        email_service,
        ws_manager.clone(),
    );

    // Start server
    let bind_address = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(notification_service.clone()))
            .app_data(web::Data::new(ws_manager.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1/notifications")
                    .route("", web::get().to(list_notifications))
                    .route("/mark-read", web::post().to(mark_notifications_read))
                    .route("/unread-count", web::get().to(get_unread_count)),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
