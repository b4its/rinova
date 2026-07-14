//! Component HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::models::{AddComponentRequest, UpdateComponentRequest, MoveComponentRequest, SaveComponentsRequest};
use crate::repository::{ComponentRepository, ProjectRepository};

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

/// GET /api/v1/projects/:id/components - Get all components for a project
pub async fn list_components(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    // Get components
    match component_repo.get_project_components(project_id).await {
        Ok(Some(components)) => HttpResponse::Ok().json(components),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project components not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get components: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// GET /api/v1/projects/:id/components/:component_id - Get a specific component
pub async fn get_component(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<(Uuid, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let (project_id, component_id) = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    // Get component
    match component_repo.get_component(project_id, &component_id).await {
        Ok(Some(component)) => HttpResponse::Ok().json(component),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Component not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get component: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// POST /api/v1/projects/:id/components - Add a component
pub async fn add_component(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    body: web::Json<AddComponentRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    // Add component
    match component_repo.add_component(project_id, body.into_inner()).await {
        Ok(component) => {
            tracing::info!(
                "Added component {} to project {} for user {}",
                component.id, project_id, user_id
            );
            HttpResponse::Created().json(serde_json::json!({
                "component": component,
                "message": "Component added successfully"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to add component: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to add component",
                "code": "ADD_FAILED"
            }))
        }
    }
}

/// PUT /api/v1/projects/:id/components/:component_id - Update a component
pub async fn update_component(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<(Uuid, String)>,
    body: web::Json<UpdateComponentRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let (project_id, component_id) = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    // Update component
    match component_repo
        .update_component(project_id, &component_id, body.into_inner())
        .await
    {
        Ok(component) => {
            tracing::info!(
                "Updated component {} in project {} for user {}",
                component_id, project_id, user_id
            );
            HttpResponse::Ok().json(serde_json::json!({
                "component": component,
                "message": "Component updated successfully"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update component: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update component",
                "code": "UPDATE_FAILED"
            }))
        }
    }
}

/// DELETE /api/v1/projects/:id/components/:component_id - Delete a component
pub async fn delete_component(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<(Uuid, String)>,
    req: HttpRequest,
) -> HttpResponse {
    let (project_id, component_id) = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    // Delete component
    match component_repo.delete_component(project_id, &component_id).await {
        Ok(deleted_ids) => {
            tracing::info!(
                "Deleted {} components from project {} for user {}",
                deleted_ids.len(),
                project_id,
                user_id
            );
            HttpResponse::Ok().json(serde_json::json!({
                "deleted_components": deleted_ids,
                "message": format!("Deleted {} component(s)", deleted_ids.len())
            }))
        }
        Err(e) => {
            tracing::error!("Failed to delete component: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete component",
                "code": "DELETE_FAILED"
            }))
        }
    }
}

/// PUT /api/v1/projects/:id/components - Bulk-save the whole component tree.
///
/// Used by the editor auto-save. Replaces the entire tree in one write.
pub async fn save_components(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<Uuid>,
    body: web::Json<SaveComponentsRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let project_id = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Confirm access first.
    match project_repo.check_access(project_id, user_id).await {
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

    // Load the project to resolve its workspace.
    let project = match project_repo.get_by_id(project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "Project not found",
                "code": "NOT_FOUND"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to load project: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }));
        }
    };

    let payload = body.into_inner();

    match component_repo
        .save_components(project_id, project.workspace_id, payload.components, payload.root_ids)
        .await
    {
        Ok(document) => {
            tracing::info!("Saved component tree for project {} (user {})", project_id, user_id);
            HttpResponse::Ok().json(document)
        }
        Err(e) => {
            tracing::error!("Failed to save components: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to save components",
                "code": "SAVE_FAILED"
            }))
        }
    }
}

/// POST /api/v1/projects/:id/components/:component_id/move - Move a component
pub async fn move_component(
    component_repo: web::Data<ComponentRepository>,
    project_repo: web::Data<ProjectRepository>,
    path: web::Path<(Uuid, String)>,
    body: web::Json<MoveComponentRequest>,
    req: HttpRequest,
) -> HttpResponse {
    #[derive(serde::Deserialize)]
    struct MoveComponentRequest {
        new_parent_id: Option<String>,
        new_position: Option<usize>,
    }

    let (project_id, component_id) = path.into_inner();
    let user_id = match get_user_id(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Check access
    match project_repo.check_access(project_id, user_id).await {
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

    let request = body.into_inner();

    // Move component
    match component_repo
        .move_component(project_id, &component_id, request.new_parent_id, request.new_position)
        .await
    {
        Ok(component) => {
            tracing::info!(
                "Moved component {} in project {} for user {}",
                component_id, project_id, user_id
            );
            HttpResponse::Ok().json(serde_json::json!({
                "component": component,
                "message": "Component moved successfully"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to move component: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to move component",
                "code": "MOVE_FAILED"
            }))
        }
    }
}
