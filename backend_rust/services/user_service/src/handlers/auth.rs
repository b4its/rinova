use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;
use shared::{errors::ServiceError, types::*};
use crate::{AppState, services};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub account_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ServiceError> {
    // Validate request
    body.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    // Validate password strength
    if !shared::utils::validate_password(&body.password) {
        return Err(ServiceError::ValidationError(
            "Password must contain uppercase, lowercase, and numbers".to_string()
        ));
    }

    // Determine account type
    let account_type = match body.account_type.as_deref() {
        Some("company") => AccountType::Company,
        _ => AccountType::Personal,
    };

    // Register user
    let user = services::register_user(
        &state.db,
        &body.email,
        &body.password,
        account_type,
        &state.zmq_publisher,
    ).await?;

    // Send verification email
    services::send_verification_email(&user.email)
        .await
        .map_err(|e| ServiceError::EmailError(e.to_string()))?;

    Ok(HttpResponse::Created().json(MessageResponse {
        message: "Registration successful. Please check your email for verification.".to_string(),
    }))
}

pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ServiceError> {
    // Login user
    let user = services::login_user(
        &state.db,
        &body.email,
        &body.password,
        &state.zmq_publisher,
    ).await?;

    // Generate JWT token
    let token = services::generate_jwt_token(&user, &state.jwt_secret)?;

    Ok(HttpResponse::Ok()
        .cookie(
            actix_web::cookie::Cookie::build("token", &token)
                .http_only(true)
                .secure(true)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::days(7))
                .finish(),
        )
        .json(AuthResponse { user, token }))
}

pub async fn verify_email(
    state: web::Data<AppState>,
    body: web::Json<VerifyEmailRequest>,
) -> Result<HttpResponse, ServiceError> {
    services::verify_user_email(&state.db, &body.token).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Email verified successfully".to_string(),
    }))
}

pub async fn resend_verification(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ServiceError> {
    services::resend_verification_email(&state.db, &body.email).await?;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Verification email sent".to_string(),
    }))
}

pub async fn logout() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok()
        .cookie(
            actix_web::cookie::Cookie::build("token", "")
                .http_only(true)
                .secure(true)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::seconds(0))
                .finish(),
        )
        .json(MessageResponse {
            message: "Logged out successfully".to_string(),
        }))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user_service"
    }))
}
