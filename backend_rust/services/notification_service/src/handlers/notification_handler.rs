//! Notification HTTP handlers

use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::models::MarkReadRequest;
use crate::services::NotificationService;

/// Extract user ID from request headers
fn get_user_id(req: &HttpRequest) -> Option<Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
}

/// GET /api/v1/notifications - List notifications
pub async fn list_notifications(
    service: web::Data<NotificationService>,
    req: HttpRequest,
    query: web::Query<ListQuery>,
) -> HttpResponse {
    let user_id = match get_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);

    match service.list_notifications(user_id, page, page_size).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            tracing::error!("Failed to list notifications: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list notifications",
                "code": "QUERY_FAILED"
            }))
        }
    }
}

/// POST /api/v1/notifications/mark-read - Mark notifications as read
pub async fn mark_notifications_read(
    service: web::Data<NotificationService>,
    req: HttpRequest,
    body: web::Json<MarkReadRequest>,
) -> HttpResponse {
    let user_id = match get_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    if body.notification_ids.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No notification IDs provided",
            "code": "INVALID_REQUEST"
        }));
    }

    match service.mark_read(user_id, &body.notification_ids).await {
        Ok(count) => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": format!("{} notifications marked as read", count),
                "count": count
            }))
        }
        Err(e) => {
            tracing::error!("Failed to mark notifications as read: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to mark notifications as read",
                "code": "UPDATE_FAILED"
            }))
        }
    }
}

/// GET /api/v1/notifications/unread-count - Get unread count
pub async fn get_unread_count(
    service: web::Data<NotificationService>,
    req: HttpRequest,
) -> HttpResponse {
    let user_id = match get_user_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "code": "UNAUTHORIZED"
            }))
        }
    };

    // Note: Would call store.get_unread_count(user_id)
    HttpResponse::Ok().json(serde_json::json!({
        "unread_count": 0
    }))
}

/// Query parameters for listing notifications
#[derive(Debug, serde::Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
