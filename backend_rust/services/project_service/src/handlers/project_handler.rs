//! Project HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use validator::Validate;

use crate::models::{CreateProjectRequest, ProjectListQuery, UpdateProjectRequest};
use crate::repository::ProjectRepository;

/// Extract user ID from request headers (set by API Gateway)
fn get_user_id(req: &HttpRequest) -> Result<Uuid, HttpResponse> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        })
}

/// Error response helper
fn error_response(message: &str, code: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": message,
        "code": code
    }))
}

/// GET /api/v1/projects - List projects
pub async fn list_projects(
    repo: web::Data<ProjectRepository>,
    query: web::Query<ProjectListQuery>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    match repo.list(query.into_inner(), user_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list projects: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/projects - Create a new project
pub async fn create_project(
    repo: web::Data<ProjectRepository>,
    body: web::Json<CreateProjectRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string(),
            "code": "VALIDATION_ERROR"
        }));
    }

    // Create project
    match repo.create(body.workspace_id, user_id, &body.name, body.metadata.clone()).await {
        Ok(project) => {
            tracing::info!("Created project {} for user {}", project.id, user_id);
            HttpResponse::Created().json(project)
        }
        Err(e) => {
            tracing::error!("Failed to create project: {}", e);
            error_response("Failed to create project", "CREATE_FAILED")
        }
    }
}

/// GET /api/v1/projects/:id - Get a project
pub async fn get_project(
    repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match repo.check_access(project_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied",
                "code": "FORBIDDEN"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check access: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    }

    // Get project
    match repo.get_by_id(project_id).await {
        Ok(Some(project)) => HttpResponse::Ok().json(project),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get project: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/projects/:id - Update a project
pub async fn update_project(
    repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProjectRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string(),
            "code": "VALIDATION_ERROR"
        }));
    }

    // Check access
    match repo.check_access(project_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied",
                "code": "FORBIDDEN"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check access: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    }

    // Update project
    match repo.update(project_id, body.into_inner()).await {
        Ok(project) => {
            tracing::info!("Updated project {} for user {}", project_id, user_id);
            HttpResponse::Ok().json(project)
        }
        Err(e) => {
            tracing::error!("Failed to update project: {}", e);
            error_response("Failed to update project", "UPDATE_FAILED")
        }
    }
}

/// DELETE /api/v1/projects/:id - Delete a project
pub async fn delete_project(
    repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match repo.check_access(project_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied",
                "code": "FORBIDDEN"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check access: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    }

    // Delete project
    match repo.delete(project_id).await {
        Ok(()) => {
            tracing::info!("Deleted project {} for user {}", project_id, user_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            tracing::error!("Failed to delete project: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete project",
                "code": "DELETE_FAILED"
            }))
        }
    }
}

/// POST /api/v1/projects/:id/archive - Archive a project
pub async fn archive_project(
    repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match repo.check_access(project_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied",
                "code": "FORBIDDEN"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check access: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    }

    // Archive project
    match repo.archive(project_id).await {
        Ok(project) => {
            tracing::info!("Archived project {} for user {}", project_id, user_id);
            HttpResponse::Ok().json(project)
        }
        Err(e) => {
            tracing::error!("Failed to archive project: {}", e);
            error_response("Failed to archive project", "ARCHIVE_FAILED")
        }
    }
}

/// GET /api/v1/projects/:id/published - Get published site info
pub async fn get_published_site(
    repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match repo.check_access(project_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Access denied",
                "code": "FORBIDDEN"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to check access: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    }

    // Get published site
    match repo.get_published_site(project_id).await {
        Ok(Some(site)) => HttpResponse::Ok().json(site),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not published",
                "code": "NOT_PUBLISHED"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get published site: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}
