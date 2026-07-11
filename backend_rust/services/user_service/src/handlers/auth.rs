use actix_web::{web, HttpResponse, get};
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

#[derive(Debug, Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
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

#[get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "user_service"
    }))
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

    // Register user (creates Personal Workspace automatically)
    let user = services::register_user(
        &state.db,
        &body.email,
        &body.password,
        account_type,
        &state.zmq_publisher,
    ).await?;

    // Generate verification token
    let verification_token = services::generate_verification_token(
        &user.id.to_string(),
        &user.email,
        &state.jwt_secret,
    )?;

    // Send verification email within 30 seconds (async, non-blocking)
    // Email sending happens in background - registration is considered successful
    let email_clone = user.email.clone();
    let token_clone = verification_token.clone();
    tokio::spawn(async move {
        if let Err(e) = services::send_verification_email(&email_clone, &token_clone).await {
            tracing::error!("Failed to send verification email: {}", e);
        }
    });

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
    let user = services::verify_user_email(&state.db, &body.token, &state.jwt_secret).await?;

    // Generate JWT token for the user to auto-login after verification
    let token = services::generate_jwt_token(&user, &state.jwt_secret)?;

    // Return response with redirect URL for onboarding
    Ok(HttpResponse::Ok()
        .cookie(
            actix_web::cookie::Cookie::build("token", &token)
                .http_only(true)
                .secure(true)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::days(7))
                .finish(),
        )
        .json(serde_json::json!({
            "message": "Email verified successfully",
            "redirect_url": "/onboarding",
            "user": user
        })))
}

pub async fn resend_verification(
    state: web::Data<AppState>,
    body: web::Json<ResendVerificationRequest>,
) -> Result<HttpResponse, ServiceError> {
    let token = services::resend_verification_email(&state.db, &body.email, &state.jwt_secret).await?;

    // Send the verification email asynchronously
    let email_clone = body.email.clone();
    let token_clone = token.clone();
    tokio::spawn(async move {
        if let Err(e) = services::send_verification_email(&email_clone, &token_clone).await {
            tracing::error!("Failed to send verification email: {}", e);
        }
    });

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Verification email sent. Please check your inbox.".to_string(),
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
