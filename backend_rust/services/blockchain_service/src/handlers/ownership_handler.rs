//! Ownership HTTP handlers

use actix_web::{web, HttpResponse};
use validator::Validate;

use crate::models::{RecordOwnershipRequest, TransferOwnershipRequest};
use crate::services::BlockchainClient;

/// POST /api/v1/blockchain/ownership - Record ownership
pub async fn record_ownership(
    client: web::Data<BlockchainClient>,
    body: web::Json<RecordOwnershipRequest>,
) -> HttpResponse {
    // Validate asset ID length
    if body.asset_id.len() > 64 || body.asset_id.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Asset ID must be 1-64 characters",
            "code": "INVALID_ASSET_ID"
        }));
    }

    // Validate Ethereum address format (0x + 40 hex chars)
    if !body.owner_address.starts_with("0x") || body.owner_address.len() != 42 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid Ethereum address format",
            "code": "INVALID_ADDRESS"
        }));
    }

    match client.record_ownership(body.into_inner()).await {
        Ok(record) => {
            tracing::info!("Ownership recorded: {}", record.asset_id);
            HttpResponse::Created().json(record)
        }
        Err(e) => {
            tracing::error!("Failed to record ownership: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to record ownership",
                "code": "RECORD_FAILED"
            }))
        }
    }
}

/// GET /api/v1/blockchain/ownership/:asset_id - Get ownership proof
pub async fn get_ownership(
    client: web::Data<BlockchainClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    match client.get_ownership(&asset_id).await {
        Ok(Some(proof)) => HttpResponse::Ok().json(proof),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Ownership not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get ownership: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get ownership",
                "code": "QUERY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/blockchain/ownership/transfer - Transfer ownership
pub async fn transfer_ownership(
    client: web::Data<BlockchainClient>,
    body: web::Json<TransferOwnershipRequest>,
) -> HttpResponse {
    // Validate addresses
    if !body.new_owner_address.starts_with("0x") || body.new_owner_address.len() != 42 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid Ethereum address format",
            "code": "INVALID_ADDRESS"
        }));
    }

    if body.signature.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Signature is required",
            "code": "MISSING_SIGNATURE"
        }));
    }

    match client.transfer_ownership(body.into_inner()).await {
        Ok(record) => {
            tracing::info!("Ownership transferred: {}", record.asset_id);
            HttpResponse::Ok().json(record)
        }
        Err(e) => {
            tracing::error!("Failed to transfer ownership: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to transfer ownership",
                "code": "TRANSFER_FAILED"
            }))
        }
    }
}

/// GET /api/v1/blockchain/ownership/:asset_id/history - Get ownership history
pub async fn get_ownership_history(
    client: web::Data<BlockchainClient>,
    path: web::Path<String>,
) -> HttpResponse {
    let asset_id = path.into_inner();

    match client.get_ownership_history(&asset_id).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => {
            tracing::error!("Failed to get ownership history: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get ownership history",
                "code": "QUERY_FAILED"
            }))
        }
    }
}
