//! Domain HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::models::AddDomainRequest;
use crate::services::DomainService;

/// Extract user ID from request headers
fn get_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

/// GET /api/v1/publish/:project_id/domains - List domains
pub async fn list_domains(
    service: web::Data<DomainService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();

    match service.list_domains(project_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list domains: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list domains",
                "code": "QUERY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/publish/:project_id/domains - Add domain
pub async fn add_domain(
    service: web::Data<DomainService>,
    path: web::Path<Uuid>,
    body: web::Json<AddDomainRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();

    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string(),
            "code": "VALIDATION_ERROR"
        }));
    }

    match service.add_domain(project_id, body.into_inner()).await {
        Ok(domain) => {
            tracing::info!("Added domain {} to project {}", domain.domain, project_id);
            HttpResponse::Created().json(domain)
        }
        Err(e) => {
            tracing::error!("Failed to add domain: {}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string(),
                "code": "ADD_FAILED"
            }))
        }
    }
}

/// DELETE /api/v1/publish/:project_id/domains/:domain_id - Remove domain
pub async fn remove_domain(
    service: web::Data<DomainService>,
    path: web::Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let (project_id, domain_id) = path.into_inner();

    match service.remove_domain(project_id, domain_id).await {
        Ok(()) => {
            tracing::info!("Removed domain {} from project {}", domain_id, project_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            tracing::error!("Failed to remove domain: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to remove domain",
                "code": "REMOVE_FAILED"
            }))
        }
    }
}

/// PUT /api/v1/publish/:project_id/domains/:domain_id/primary - Set primary
pub async fn set_primary_domain(
    service: web::Data<DomainService>,
    path: web::Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let (project_id, domain_id) = path.into_inner();

    match service.set_primary(project_id, domain_id).await {
        Ok(domain) => {
            tracing::info!("Set domain {} as primary for project {}", domain_id, project_id);
            HttpResponse::Ok().json(domain)
        }
        Err(e) => {
            tracing::error!("Failed to set primary domain: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to set primary domain",
                "code": "SET_PRIMARY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/publish/:project_id/domains/:domain_id/verify - Verify DNS
pub async fn verify_domain_dns(
    service: web::Data<DomainService>,
    path: web::Path<(Uuid, String)>,
) -> HttpResponse {
    let (project_id, domain) = path.into_inner();

    match service.verify_dns(project_id, &domain).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to verify DNS: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to verify DNS",
                "code": "VERIFY_FAILED"
            }))
        }
    }
}

/// GET /api/v1/publish/:project_id/domains/:domain_id/ssl - Get SSL status
pub async fn get_domain_ssl(
    service: web::Data<DomainService>,
    path: web::Path<(Uuid, String)>,
) -> HttpResponse {
    let (project_id, domain) = path.into_inner();

    match service.get_ssl_status(project_id, &domain).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to get SSL status: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get SSL status",
                "code": "QUERY_FAILED"
            }))
        }
    }
}
