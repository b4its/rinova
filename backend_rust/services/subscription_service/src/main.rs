//! Subscription Service
//! 
//! Microservice for subscription plan management, feature flagging, and billing integration.
//! 
//! ## Endpoints
//! 
//! - GET    /health                      - Health check
//! - GET    /api/v1/subscriptions/plans  - List all available plans
//! - GET    /api/v1/subscriptions/me     - Get current user's subscription
//! - POST   /api/v1/subscriptions/upgrade - Upgrade subscription plan
//! - POST   /api/v1/subscriptions/downgrade - Downgrade subscription plan
//! - POST   /api/v1/subscriptions/:id/cancel - Cancel subscription

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod handlers;
mod models;
mod repository;
mod services;

use handlers::*;
use repository::SubscriptionRepository;
use services::{StripeService, SubscriptionService};

/// Service configuration
#[derive(Debug, Deserialize, Clone)]
struct Settings {
    server: ServerSettings,
    database: DatabaseSettings,
    stripe: StripeSettings,
}

#[derive(Debug, Deserialize, Clone)]
struct ServerSettings {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, Clone)]
struct DatabaseSettings {
    url: String,
    max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct StripeSettings {
    secret_key: String,
    webhook_secret: String,
}

impl Settings {
    fn from_env() -> Result<Self, ConfigError> {
        Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8002)?
            .set_default("database.max_connections", 5)?
            .add_source(File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize()
    }
}

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
        service: "subscription_service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    // Load configuration
    let settings = Settings::from_env().expect("Failed to load configuration");
    tracing::info!("Loaded configuration: server={}:{}", settings.server.host, settings.server.port);

    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Connected to database");

    // Initialize Stripe service
    let stripe_service = if !settings.stripe.secret_key.is_empty() {
        Some(StripeService::new(&settings.stripe.secret_key))
    } else {
        tracing::warn!("Stripe not configured - payment features disabled");
        None
    };

    // Create repository and service
    let repo = SubscriptionRepository::new(pool.clone());
    let subscription_service = Arc::new(SubscriptionService::new(repo, stripe_service));

    // Run migrations if needed
    // sqlx::migrate!().run(&pool).await.expect("Failed to run migrations");

    // Start server
    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    tracing::info!("Starting Subscription Service on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(subscription_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1/subscriptions")
                    .route("/plans", web::get().to(list_plans))
                    .route("/{user_id}", web::get().to(get_subscription))
                    .route("/{user_id}/upgrade", web::post().to(upgrade_subscription))
                    .route("/{user_id}/downgrade", web::post().to(downgrade_subscription))
                    .route("/{id}/cancel", web::post().to(cancel_subscription))
                    .route("/{user_id}/project-limit", web::get().to(check_project_limit))
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
