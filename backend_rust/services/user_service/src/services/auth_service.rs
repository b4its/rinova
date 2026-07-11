use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Duration, Utc};
use shared::{errors::ServiceError, types::{User, AccountType, Claims}};
use shared::ServiceResult;
use database::DbPool;
use messaging::ZeroMQPublisher;
use crate::repository;

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

    // Hash password
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| ServiceError::InternalError)?;

    // Create user
    let user = repository::create(pool, email, &password_hash, account_type).await?;

    // Publish user registered event
    zmq_publisher.publish("user.registered", serde_json::json!({
        "user_id": user.id,
        "email": user.email,
        "account_type": user.account_type,
    })).await;

    tracing::info!("User registered: {}", user.email);

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

pub async fn verify_user_email(pool: &DbPool, token: &str) -> ServiceResult<()> {
    // In real implementation, decode token and verify
    // For now, we'll use a simplified version
    let user_id = uuid::Uuid::parse_str(token)
        .map_err(|_| ServiceError::TokenInvalid)?;

    repository::verify_email(pool, user_id).await?;

    tracing::info!("User email verified: {}", user_id);

    Ok(())
}

pub async fn resend_verification_email(pool: &DbPool, email: &str) -> ServiceResult<()> {
    let user = repository::find_by_email(pool, email)
        .await?
        .ok_or(ServiceError::NotFoundError("User not found".to_string()))?;

    if user.email_verified_at.is_some() {
        return Err(ServiceError::ValidationError("Email already verified".to_string()));
    }

    // In real implementation, send email with verification link
    // For now, we'll just return success
    tracing::info!("Resending verification email to: {}", email);

    Ok(())
}
