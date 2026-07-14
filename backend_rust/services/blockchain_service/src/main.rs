//! Blockchain Service
//!
//! Microservice for blockchain integration with Hyperledger Besu.
//!
//! ## Features
//!
//! - Asset ownership recording and verification (ERC-721)
//! - Immutable audit trail for publish events
//! - SHA-256 hash computation for content verification
//!
//! ## Endpoints
//!
//! - POST   /api/v1/blockchain/ownership              - Record ownership
//! - GET    /api/v1/blockchain/ownership/:asset_id    - Get ownership proof
//! - POST   /api/v1/blockchain/ownership/transfer     - Transfer ownership
//! - GET    /api/v1/blockchain/ownership/:asset_id/history - Get history
//! - POST   /api/v1/blockchain/audit/:project_id     - Record publish
//! - GET    /api/v1/blockchain/audit/:project_id     - Get audit trail
//! - GET    /api/v1/blockchain/audit/:project_id/verify - Verify hash
//! - POST   /api/v1/blockchain/hash                   - Compute hash

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;

mod grpc;
mod handlers;
mod models;
mod services;

use grpc::BlockchainGrpc;
use handlers::*;
use proto::blockchain::blockchain_service_server::BlockchainServiceServer;
use services::{BlockchainClient, BlockchainConfig, HashService};

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    blockchain_connected: bool,
}

/// Health check endpoint
async fn health(client: web::Data<BlockchainClient>) -> web::Json<HealthResponse> {
    let connected = client.health_check().await.unwrap_or(false);
    
    web::Json(HealthResponse {
        status: if connected { "healthy" } else { "degraded" }.to_string(),
        service: "blockchain_service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        blockchain_connected: connected,
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
    let port = std::env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8005".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    // Initialize blockchain client
    let config = BlockchainConfig::from_env()
        .expect("Failed to load blockchain configuration");
    
    let blockchain_client = BlockchainClient::new(config)
        .await
        .expect("Failed to initialize blockchain client");

    tracing::info!("Blockchain client initialized");

    // Initialize hash service
    let hash_service = HashService::new();

    // Start the gRPC server on a separate port for internal service calls.
    let grpc_port = std::env::var("GRPC_PORT")
        .unwrap_or_else(|_| "9005".to_string())
        .parse::<u16>()
        .expect("GRPC_PORT must be a valid port number");
    let grpc_client = blockchain_client.clone();
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", grpc_port).parse().expect("valid grpc addr");
        tracing::info!("Starting Blockchain gRPC server on {}", addr);
        let svc = BlockchainServiceServer::new(BlockchainGrpc::new(grpc_client));
        if let Err(e) = tonic::transport::Server::builder()
            .add_service(svc)
            .serve(addr)
            .await
        {
            tracing::error!("gRPC server error: {}", e);
        }
    });

    // Start server
    let bind_address = format!("0.0.0.0:{}", port);
    tracing::info!("Starting Blockchain Service on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(blockchain_client.clone()))
            .app_data(web::Data::new(hash_service.clone()))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api/v1/blockchain/ownership")
                    .route("", web::post().to(record_ownership))
                    .route("/{asset_id}", web::get().to(get_ownership))
                    .route("/transfer", web::post().to(transfer_ownership))
                    .route("/{asset_id}/history", web::get().to(get_ownership_history)),
            )
            .service(
                web::scope("/api/v1/blockchain/audit")
                    .route("/{project_id}", web::post().to(record_publish))
                    .route("/{project_id}", web::get().to(get_audit_trail))
                    .route("/{project_id}/verify", web::get().to(verify_audit)),
            )
            .service(
                web::scope("/api/v1/blockchain/subscription")
                    .route("", web::post().to(record_subscription))
                    .route("/{user_id}", web::get().to(get_subscription_history)),
            )
            .route("/api/v1/blockchain/hash", web::post().to(compute_hash))
    })
    .bind(&bind_address)?
    .run()
    .await
}
