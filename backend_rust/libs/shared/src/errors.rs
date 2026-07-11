use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("Account locked")]
    AccountLocked,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Permission denied: {0}")]
    ForbiddenError(String),

    #[error("Subscription limit reached: {0}")]
    SubscriptionLimitError(String),

    #[error("Feature locked for current plan")]
    FeatureLocked,

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Internal server error")]
    InternalError,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Email sending failed: {0}")]
    EmailError(String),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ServiceError::NotFoundError("Resource not found".to_string()),
            _ => ServiceError::DatabaseError(err.to_string()),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let error_code = self.to_code();
        let status_code = self.status_code();

        HttpResponse::build(status_code).json(ApiErrorResponse {
            code: error_code,
            message: self.to_string(),
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}

impl ServiceError {
    fn to_code(&self) -> &'static str {
        match self {
            ServiceError::AuthError(_) => "AUTH_ERROR",
            ServiceError::InvalidCredentials => "AUTH_INVALID_CREDENTIALS",
            ServiceError::TokenExpired => "AUTH_TOKEN_EXPIRED",
            ServiceError::TokenInvalid => "AUTH_TOKEN_INVALID",
            ServiceError::EmailNotVerified => "AUTH_EMAIL_NOT_VERIFIED",
            ServiceError::AccountLocked => "AUTH_ACCOUNT_LOCKED",
            ServiceError::ValidationError(_) => "VAL_ERROR",
            ServiceError::NotFoundError(_) => "RES_NOT_FOUND",
            ServiceError::ForbiddenError(_) => "RES_FORBIDDEN",
            ServiceError::SubscriptionLimitError(_) => "SUB_PLAN_LIMIT_REACHED",
            ServiceError::FeatureLocked => "SUB_FEATURE_LOCKED",
            ServiceError::PaymentFailed(_) => "SUB_PAYMENT_FAILED",
            ServiceError::RateLimitError => "RATE_LIMIT_EXCEEDED",
            ServiceError::InternalError => "SYS_INTERNAL_ERROR",
            ServiceError::DatabaseError(_) => "SYS_DATABASE_ERROR",
            ServiceError::EmailError(_) => "SYS_EMAIL_ERROR",
            ServiceError::BlockchainError(_) => "BLK_ERROR",
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            ServiceError::AuthError(_)
            | ServiceError::InvalidCredentials
            | ServiceError::TokenExpired
            | ServiceError::TokenInvalid
            | ServiceError::EmailNotVerified
            | ServiceError::AccountLocked => StatusCode::UNAUTHORIZED,

            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFoundError(_) => StatusCode::NOT_FOUND,
            ServiceError::ForbiddenError(_) => StatusCode::FORBIDDEN,

            ServiceError::SubscriptionLimitError(_)
            | ServiceError::FeatureLocked => StatusCode::PAYMENT_REQUIRED,

            ServiceError::RateLimitError => StatusCode::TOO_MANY_REQUESTS,

            ServiceError::DatabaseError(_)
            | ServiceError::InternalError
            | ServiceError::EmailError(_)
            | ServiceError::BlockchainError(_) => StatusCode::INTERNAL_SERVER_ERROR,

            ServiceError::PaymentFailed(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ApiErrorResponse {
    pub code: &'static str,
    pub message: String,
    pub request_id: String,
    pub timestamp: String,
}

pub type ServiceResult<T> = Result<T, ServiceError>;
