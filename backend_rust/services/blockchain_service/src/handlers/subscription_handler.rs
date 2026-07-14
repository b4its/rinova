//! Subscription transaction blockchain handlers

use actix_web::{web, HttpResponse};

use crate::models::RecordSubscriptionRequest;
use crate::services::BlockchainClient;

/// POST /api/v1/blockchain/subscription - Record a subscription transaction
pub async fn record_subscription(
    client: web::Data<BlockchainClient>,
    body: web::Json<RecordSubscriptionRequest>,
) -> HttpResponse {
    let req = body.into_inner();

    if req.user_id.is_empty() || req.subscription_id.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "user_id and subscription_id are required",
            "code": "MISSING_FIELDS"
        }));
    }

    match client.record_subscription(req).await {
        Ok(record) => {
            tracing::info!("Subscription tx recorded: {}", record.tx_hash);
            HttpResponse::Created().json(record)
        }
        Err(e) => {
            tracing::error!("Failed to record subscription tx: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to record subscription transaction",
                "code": "RECORD_FAILED"
            }))
        }
    }
}

/// GET /api/v1/blockchain/subscription/{user_id} - Get subscription tx history
pub async fn get_subscription_history(
    client: web::Data<BlockchainClient>,
    path: web::Path<String>,
    query: web::Query<SubscriptionHistoryQuery>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let limit = query.limit.unwrap_or(100);

    match client.get_subscription_history(&user_id, limit).await {
        Ok(records) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": user_id,
            "records": records,
            "total": records.len()
        })),
        Err(e) => {
            tracing::error!("Failed to get subscription history: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get subscription history",
                "code": "QUERY_FAILED"
            }))
        }
    }
}

/// Query params for subscription history
#[derive(Debug, serde::Deserialize)]
pub struct SubscriptionHistoryQuery {
    pub limit: Option<u64>,
}
