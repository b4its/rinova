use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Duration, Utc};
use shared::{errors::ServiceError, types::{User, AccountType, Claims}};
use shared::ServiceResult;
use database::DbPool;
use messaging::ZeroMQPublisher;
use serde::{Deserialize, Serialize};
use crate::repository;

const BCRYPT_COST: u32 = 12;
const VERIFICATION_TOKEN_EXPIRY_HOURS: i64 = 24;

/// Claims for email verification token
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationClaims {
    pub sub: String,      // user_id
    pub email: String,    // user email
    pub exp: usize,       // expiration timestamp
    pub iat: usize,       // issued at timestamp
    pub purpose: String,  // "email_verification"
}

pub async fn register_user(
    pool: &DbPool,
    email: &str,
    password: &str,
    account_type: AccountType,
    zmq_publisher: &ZeroMQPublisher,
) -> ServiceResult<User> {
    // Check if email already exists
    if repository::check_email_exists(pool, email).await? {
        return Err(ServiceError::ValidationError("Email already registered".to_string()));
    }

    // Hash password with bcrypt cost factor 12
    let password_hash = hash(password, BCRYPT_COST)
        .map_err(|_| ServiceError::InternalError)?;

    // Create user
    let user = repository::create(pool, email, &password_hash, account_type.clone()).await?;

    // Create Personal Workspace for the user
    let workspace_id = repository::create_personal_workspace(pool, user.id).await?;

    // Publish user registered event
    zmq_publisher.publish("user.registered", serde_json::json!({
        "user_id": user.id,
        "email": user.email,
        "account_type": user.account_type,
        "workspace_id": workspace_id,
    })).await;

    tracing::info!("User registered: {} with workspace: {}", user.email, workspace_id);

    Ok(user)
}

pub async fn login_user(
    pool: &DbPool,
    email: &str,
    password: &str,
    zmq_publisher: &ZeroMQPublisher,
) -> ServiceResult<User> {
    // Find user by email
    let user = repository::find_by_email(pool, email)
        .await?
        .ok_or(ServiceError::InvalidCredentials)?;

    // Verify password
    let valid = verify(password, &user.password_hash)
        .map_err(|_| ServiceError::InternalError)?;

    if !valid {
        return Err(ServiceError::InvalidCredentials);
    }

    // Check if email verified
    if user.email_verified_at.is_none() {
        return Err(ServiceError::EmailNotVerified);
    }

    // Publish login event
    zmq_publisher.publish("user.logged_in", serde_json::json!({
        "user_id": user.id,
        "email": user.email,
    })).await;

    tracing::info!("User logged in: {}", user.email);

    Ok(user)
}

pub fn generate_jwt_token(user: &User, secret: &str) -> ServiceResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ServiceError::InternalError)
}

pub fn verify_jwt_token(token: &str, secret: &str) -> ServiceResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ServiceError::TokenInvalid)
}

/// Generate email verification token with 24-hour expiry
pub fn generate_verification_token(user_id: &str, email: &str, secret: &str) -> ServiceResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(VERIFICATION_TOKEN_EXPIRY_HOURS))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = VerificationClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        purpose: "email_verification".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ServiceError::InternalError)
}

/// Verify email verification token
pub fn verify_verification_token(token: &str, secret: &str) -> ServiceResult<VerificationClaims> {
    let claims = decode::<VerificationClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        // Check if token is expired vs invalid
        if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = e.kind() {
            ServiceError::TokenExpired
        } else {
            ServiceError::TokenInvalid
        }
    })?;

    // Verify purpose
    if claims.purpose != "email_verification" {
        return Err(ServiceError::TokenInvalid);
    }

    Ok(claims)
}

pub async fn verify_user_email(pool: &DbPool, token: &str, secret: &str) -> ServiceResult<User> {
    // Verify and decode the token
    let claims = verify_verification_token(token, secret)?;

    // Parse user_id from claims
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ServiceError::TokenInvalid)?;

    // Find user to verify they exist and check if already verified
    let user = repository::find_by_id(pool, user_id)
        .await?
        .ok_or(ServiceError::NotFoundError("User not found".to_string()))?;

    if user.email_verified_at.is_some() {
        return Err(ServiceError::ValidationError("Email already verified".to_string()));
    }

    // Verify the email matches the token
    if user.email != claims.email {
        return Err(ServiceError::TokenInvalid);
    }

    // Update user's email_verified_at
    repository::verify_email(pool, user_id).await?;

    // Return the updated user
    let updated_user = repository::find_by_id(pool, user_id)
        .await?
        .ok_or(ServiceError::InternalError)?;

    tracing::info!("User email verified: {}", user_id);

    Ok(updated_user)
}

pub async fn resend_verification_email(pool: &DbPool, email: &str, secret: &str) -> ServiceResult<String> {
    let user = repository::find_by_email(pool, email)
        .await?
        .ok_or(ServiceError::NotFoundError("User not found".to_string()))?;

    if user.email_verified_at.is_some() {
        return Err(ServiceError::ValidationError("Email already verified".to_string()));
    }

    // Generate new verification token
    let token = generate_verification_token(&user.id.to_string(), &user.email, secret)?;

    tracing::info!("Generated new verification token for: {}", email);

    Ok(token)
}
