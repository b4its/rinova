//! Project Service
//!
//! Microservice for project and component management.
//!
//! ## Endpoints
//!
//! - GET    /health                          - Health check
//! - GET    /api/v1/projects                 - List projects
//! - POST   /api/v1/projects                 - Create project
//! - GET    /api/v1/projects/:id             - Get project
//! - PUT    /api/v1/projects/:id             - Update project
//! - DELETE /api/v1/projects/:id             - Delete project
//! - POST   /api/v1/projects/:id/archive     - Archive project
//! - GET    /api/v1/projects/:id/published   - Get published site info
//! - GET    /api/v1/projects/:id/components  - List components
//! - POST   /api/v1/projects/:id/components  - Add component
//! - GET    /api/v1/projects/:id/components/:cid - Get component
//! - PUT    /api/v1/projects/:id/components/:cid - Update component
//! - DELETE /api/v1/projects/:id/components/:cid - Delete component
//! - POST   /api/v1/projects/:id/components/:cid/move - Move component

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;
mod repository;
mod services;

use handlers::*;
use repository::{ComponentRepository, ProjectRepository};

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
        service: "project_service".to_string(),
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
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let mongodb_database = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "rinova".to_string());
    let port = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8003".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    tracing::info!("Starting Project Service on port {}", port);

    // Connect to PostgreSQL
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    // Create PostgreSQL repository
    let project_repo = ProjectRepository::new(pg_pool.clone());

    // Connect to MongoDB and create component repository
    // Note: ComponentRepository is created once and cloned for each worker
    let component_repo = std::sync::Arc::new(
        ComponentRepository::new(&mongodb_uri, &mongodb_database)
            .await
            .expect("Failed to create component repository")
    );

    tracing::info!("Connected to MongoDB");

    // Start server
    let bind_address = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(project_repo.clone()))
            .app_data(web::Data::from(component_repo.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1/projects")
                    // Project CRUD
                    .route("", web::get().to(list_projects))
                    .route("", web::post().to(create_project))
                    .route("/{id}", web::get().to(get_project))
                    .route("/{id}", web::put().to(update_project))
                    .route("/{id}", web::delete().to(delete_project))
                    .route("/{id}/archive", web::post().to(archive_project))
                    .route("/{id}/published", web::get().to(get_published_site))
                    // Component management
                    .route("/{id}/components", web::get().to(list_components))
                    .route("/{id}/components", web::post().to(add_component))
                    .route("/{id}/components", web::put().to(save_components))
                    .route("/{id}/components/{component_id}", web::get().to(get_component))
                    .route("/{id}/components/{component_id}", web::put().to(update_component))
                    .route("/{id}/components/{component_id}", web::delete().to(delete_component))
                    .route(
                        "/{id}/components/{component_id}/move",
                        web::post().to(move_component),
                    ),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
