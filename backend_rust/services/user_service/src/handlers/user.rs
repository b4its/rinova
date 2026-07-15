use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{repository, AppState};

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub account_type: shared::types::AccountType,
    pub role: String,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<shared::types::User> for UserResponse {
    fn from(user: shared::types::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            account_type: user.account_type,
            role: match user.role {
                shared::types::UserRole::Superuser => "superuser".to_string(),
                shared::types::UserRole::User => "user".to_string(),
            },
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
}

/// Extract the authenticated user id from the `X-User-ID` header injected by
/// the API gateway after it validates the JWT.
fn extract_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

/// Extract the user's role from the `X-User-Role` header injected by the API gateway.
fn extract_user_role(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("X-User-Role")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: String,
}

/// GET /api/v1/admin/users - list all users (admin only).
pub async fn admin_list_users(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let role = match extract_user_role(&req) {
        Some(r) => r,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    if role != "superuser" {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required",
            "code": "FORBIDDEN"
        }))
    }

    match repository::list_all(&state.db).await {
        Ok(users) => {
            let resp: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => {
            tracing::error!("Failed to list users: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/admin/users/{id}/role - update a user's role (admin only).
pub async fn admin_update_user_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserRoleRequest>,
) -> HttpResponse {
    let role = match extract_user_role(&req) {
        Some(r) => r,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    if role != "superuser" {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required",
            "code": "FORBIDDEN"
        }))
    }

    let user_id = path.into_inner();

    match repository::update_role(&state.db, user_id, &body.role).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(e) => {
            tracing::error!("Failed to update user role {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user role",
                "code": "UPDATE_FAILED"
            }))
        }
    }
}

/// DELETE /api/v1/admin/users/{id} - delete a user (admin only).
pub async fn admin_delete_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let role = match extract_user_role(&req) {
        Some(r) => r,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    if role != "superuser" {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required",
            "code": "FORBIDDEN"
        }))
    }

    let user_id = path.into_inner();

    match repository::delete_user(&state.db, user_id).await {
        Ok(true) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User deleted successfully",
            "code": "DELETED"
        })),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found",
            "code": "USER_NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to delete user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete user",
                "code": "DELETE_FAILED"
            }))
        }
    }
}

/// GET /api/v1/users/me - return the authenticated user's profile.
pub async fn get_current_user(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    match repository::find_by_id(&state.db, user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponse::from(user)),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found",
            "code": "USER_NOT_FOUND"
        })),
        Err(e) => {
            tracing::error!("Failed to load user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error",
                "code": "INTERNAL_ERROR"
            }))
        }
    }
}

/// PUT /api/v1/users/me - update the authenticated user's profile.
pub async fn update_user(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let user_id = match extract_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    match repository::update(&state.db, user_id, body.full_name.as_deref()).await {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(e) => {
            tracing::error!("Failed to update user {}: {}", user_id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user",
                "code": "UPDATE_FAILED"
            }))
        }
    }
}
