use actix_web::{HttpResponse};

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub account_type: shared::types::AccountType,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<shared::types::User> for UserResponse {
    fn from(user: shared::types::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            account_type: user.account_type,
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}

pub async fn get_current_user() -> HttpResponse {
    // This is a placeholder - actual implementation requires JWT middleware
    HttpResponse::Ok().json(serde_json::json!({
        "message": "User endpoint - requires authentication"
    }))
}

pub async fn update_user() -> HttpResponse {
    // This is a placeholder - actual implementation requires JWT middleware
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Update user endpoint - requires authentication"
    }))
}
