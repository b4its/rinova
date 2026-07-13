//! Audit trail HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::models::RecordPublishRequest;
use crate::services::{BlockchainClient, HashService};

/// Extract user ID from request headers
fn get_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

/// POST /api/v1/blockchain/audit/:project_id - Record publish
pub async fn record_publish(
    client: web::Data<BlockchainClient>,
    hash_service: web::Data<HashService>,
    path: web::Path<String>,
    mut body: web::Json<RecordPublishRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();

    // Ensure project_id matches
    body.project_id = project_id.clone();

    // Compute hash if content bundle provided
    if body.content_hash.is_none() {
        if let Some(ref bundle) = body.content_bundle {
            let hash_result = hash_service.compute_hash_with_timing(bundle);
            body.content_hash = Some(hash_result.hash);
            
            tracing::info!(
                "Computed hash for project {} in {}ms ({} bytes)",
                project_id,
                hash_result.computation_time_ms,
                hash_result.content_size
            );

            // Verify computation time is within limit (< 10 seconds)
            if hash_result.computation_time_ms > 10000 {
                tracing::warn!("Hash computation took too long: {}ms", hash_result.computation_time_ms);
            }
        }
    }

    match client.record_publish(body.into_inner()).await {
        Ok(record) => {
            tracing::info!("Publish recorded for project: {}", project_id);
            HttpResponse::Created().json(record)
        }
        Err(e) => {
            tracing::error!("Failed to record publish: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to record publish",
                "code": "RECORD_FAILED"
            }))
        }
    }
}

/// GET /api/v1/blockchain/audit/:project_id - Get publish history
pub async fn get_audit_trail(
    client: web::Data<BlockchainClient>,
    path: web::Path<String>,
    query: web::Query<AuditQuery>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let limit = query.limit.unwrap_or(100);

    match client.get_publish_history(&project_id, limit).await {
        Ok(records) => {
            HttpResponse::Ok().json(serde_json::json!({
                "project_id": project_id,
                "records": records,
                "total": records.len()
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get audit trail: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get audit trail",
                "code": "QUERY_FAILED"
            }))
        }
    }
}

/// GET /api/v1/blockchain/audit/:project_id/verify - Verify content hash
pub async fn verify_audit(
    client: web::Data<BlockchainClient>,
    path: web::Path<String>,
    query: web::Query<VerifyQuery>,
) -> HttpResponse {
    let project_id = path.into_inner();

    if query.hash.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Hash parameter is required",
            "code": "MISSING_HASH"
        }));
    }

    match client.verify_hash(&project_id, &query.hash).await {
        Ok(verification) => {
            if verification.verified {
                HttpResponse::Ok().json(verification)
            } else {
                // Hash mismatch - show warning
                let mut response = serde_json::to_value(&verification).unwrap();
                if let Some(obj) = response.as_object_mut() {
                    obj.insert("warning".to_string(), serde_json::json!(
                        "Content hash does not match blockchain record. Website may have been modified without authorization."
                    ));
                }
                HttpResponse::Ok().json(response)
            }
        }
        Err(e) => {
            tracing::error!("Failed to verify hash: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify hash",
                "code": "VERIFY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/blockchain/hash - Compute hash for content
pub async fn compute_hash(
    hash_service: web::Data<HashService>,
    body: web::Json<ComputeHashRequest>,
) -> HttpResponse {
    if body.content.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Content cannot be empty",
            "code": "EMPTY_CONTENT"
        }));
    }

    // Check content size limit (100MB)
    if body.content.len() > 100 * 1024 * 1024 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Content exceeds 100MB limit",
            "code": "CONTENT_TOO_LARGE"
        }));
    }

    let result = hash_service.compute_hash_with_timing(&body.content);

    HttpResponse::Ok().json(result)
}

/// Query parameters for audit trail
#[derive(Debug, serde::Deserialize)]
pub struct AuditQuery {
    pub limit: Option<u64>,
}

/// Query parameters for verification
#[derive(Debug, serde::Deserialize)]
pub struct VerifyQuery {
    pub hash: String,
}

/// Request to compute hash
#[derive(Debug, serde::Deserialize)]
pub struct ComputeHashRequest {
    pub content: Vec<u8>,
}
