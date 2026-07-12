//! Publish HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::models::PublishRequest;
use crate::services::PublishService;

/// Extract user ID from request headers
fn get_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

/// POST /api/v1/publish/:project_id - Start publish job
pub async fn start_publish(
    service: web::Data<PublishService>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    let request = PublishRequest {
        project_id,
        skip_validation: false,
    };

    match service.start_publish(request, user_id).await {
        Ok(job) => {
            tracing::info!("Started publish job {} for project {}", job.id, project_id);
            HttpResponse::Accepted().json(job)
        }
        Err(e) => {
            tracing::error!("Failed to start publish: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to start publish",
                "code": "PUBLISH_START_FAILED"
            }))
        }
    }
}

/// GET /api/v1/publish/:project_id/status - Get publish status
pub async fn get_publish_status(
    service: web::Data<PublishService>,
    path: web::Path<Uuid>,
    query: web::Query<StatusQuery>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let job_id = match query.job_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "job_id query parameter is required",
                "code": "MISSING_JOB_ID"
            }))
        }
    };

    match service.get_status(job_id).await {
        Ok(Some(status)) => HttpResponse::Ok().json(status),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Publish job not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get publish status: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get publish status",
                "code": "STATUS_QUERY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/publish/:project_id/cancel - Cancel publish job
pub async fn cancel_publish(
    service: web::Data<PublishService>,
    path: web::Path<Uuid>,
    query: web::Query<StatusQuery>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let job_id = match query.job_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "job_id query parameter is required",
                "code": "MISSING_JOB_ID"
            }))
        }
    };

    match service.cancel_job(job_id).await {
        Ok(()) => {
            tracing::info!("Cancelled publish job {} for project {}", job_id, project_id);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Publish job cancelled",
                "job_id": job_id
            }))
        }
        Err(e) => {
            tracing::error!("Failed to cancel publish: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to cancel publish",
                "code": "CANCEL_FAILED"
            }))
        }
    }
}

/// Query parameters for status endpoint
#[derive(Debug, serde::Deserialize)]
pub struct StatusQuery {
    pub job_id: Option<Uuid>,
}
