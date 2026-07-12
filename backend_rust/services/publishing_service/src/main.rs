//! Publishing Service
//!
//! Microservice for one-click publish and domain management.
//!
//! ## Features
//!
//! - One-click website publishing
//! - Build pipeline with retry logic
//! - Custom domain management
//! - SSL certificate provisioning
//! - DNS verification
//!
//! ## Endpoints
//!
//! - POST   /api/v1/publish/:project_id           - Start publish
//! - GET    /api/v1/publish/:project_id/status    - Get publish status
//! - POST   /api/v1/publish/:project_id/cancel    - Cancel publish
//! - GET    /api/v1/publish/:project_id/domains   - List domains
//! - POST   /api/v1/publish/:project_id/domains   - Add domain
//! - DELETE /api/v1/publish/:project_id/domains/:id - Remove domain
//! - PUT    /api/v1/publish/:project_id/domains/:id/primary - Set primary
//! - POST   /api/v1/publish/:project_id/domains/:domain/verify - Verify DNS
//! - GET    /api/v1/publish/:project_id/domains/:domain/ssl - Get SSL status

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;
mod services;

use handlers::*;
use services::{DomainService, PublishService};

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

/// Health check endpoint
async fn health() -> web::Json<HealthResponse> {
    web::Json(HealthResponse {
        status: "healthy".to_string(),
        service: "publishing_service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8004".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    tracing::info!("Starting Publishing Service on port {}", port);

    // Connect to PostgreSQL
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    // Initialize services
    let publish_service = PublishService::new(pg_pool.clone());
    let domain_service = DomainService::new();

    // Start server
    let bind_address = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(publish_service.clone()))
            .app_data(web::Data::new(domain_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1/publish")
                    // Publish operations
                    .route("/{project_id}", web::post().to(start_publish))
                    .route("/{project_id}/status", web::get().to(get_publish_status))
                    .route("/{project_id}/cancel", web::post().to(cancel_publish))
                    // Domain operations
                    .route("/{project_id}/domains", web::get().to(list_domains))
                    .route("/{project_id}/domains", web::post().to(add_domain))
                    .route("/{project_id}/domains/{domain_id}", web::delete().to(remove_domain))
                    .route(
                        "/{project_id}/domains/{domain_id}/primary",
                        web::put().to(set_primary_domain),
                    )
                    .route(
                        "/{project_id}/domains/{domain}/verify",
                        web::post().to(verify_domain_dns),
                    )
                    .route(
                        "/{project_id}/domains/{domain}/ssl",
                        web::get().to(get_domain_ssl),
                    ),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
