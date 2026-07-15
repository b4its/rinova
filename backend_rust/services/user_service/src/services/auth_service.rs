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

/// Create freemium subscription for a new user via HTTP call to subscription service
async fn create_freemium_subscription(user_id: uuid::Uuid) -> Result<(), String> {
    let subscription_service_url = std::env::var("SUBSCRIPTION_SERVICE_URL")
        .unwrap_or_else(|_| "http://subscription-service:8002".to_string());

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/v1/subscriptions/freemium", subscription_service_url))
        .json(&serde_json::json!({ "user_id": user_id }))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            tracing::info!("Created freemium subscription for user: {}", user_id);
            Ok(())
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::warn!("Failed to create subscription: {} - {}", status, body);
            Err(format!("Subscription service returned: {}", status))
        }
        Err(e) => {
            tracing::error!("Failed to call subscription service: {}", e);
            Err(e.to_string())
        }
    }
}

/// Create workspace freemium subscription for a company registration
async fn create_workspace_freemium_subscription(user_id: uuid::Uuid, workspace_id: uuid::Uuid) -> Result<(), String> {
    let subscription_service_url = std::env::var("SUBSCRIPTION_SERVICE_URL")
        .unwrap_or_else(|_| "http://subscription-service:8002".to_string());

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/v1/subscriptions/workspace/freemium", subscription_service_url))
        .json(&serde_json::json!({
            "user_id": user_id,
            "workspace_id": workspace_id,
        }))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            tracing::info!("Created workspace freemium subscription for user: {} workspace: {}", user_id, workspace_id);
            Ok(())
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::warn!("Failed to create workspace subscription: {} - {}", status, body);
            Err(format!("Subscription service returned: {}", status))
        }
        Err(e) => {
            tracing::error!("Failed to call subscription service: {}", e);
            Err(e.to_string())
        }
    }
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

    // Create workspace and subscription based on account type
    let is_company = matches!(account_type, AccountType::Company);

    let workspace_id = if is_company {
        repository::create_company_workspace(pool, user.id).await?
    } else {
        repository::create_personal_workspace(pool, user.id).await?
    };

    // Create freemium subscription (best-effort, non-blocking)
    let user_id = user.id;

    tokio::spawn(async move {
        if is_company {
            // For company registrations, create a workspace subscription
            if let Err(e) = create_workspace_freemium_subscription(user_id, workspace_id).await {
                tracing::error!(
                    "Failed to create workspace freemium subscription for user {}: {}",
                    user_id, e
                );
            }
        } else {
            // Personal registration: create personal freemium subscription
            if let Err(e) = create_freemium_subscription(user_id).await {
                tracing::error!(
                    "Failed to create freemium subscription for user {}: {}",
                    user_id, e
                );
            }
        }
    });

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
) -> ServiceResult<(User, String)> {
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

    // Resolve effective plan (max of personal + workspace subscriptions)
    let plan = resolve_effective_plan(pool, user.id).await;

    // Publish login event
    zmq_publisher.publish("user.logged_in", serde_json::json!({
        "user_id": user.id,
        "email": user.email,
        "plan": plan,
    })).await;

    tracing::info!("User logged in: {} (plan: {})", user.email, plan);

    Ok((user, plan))
}

/// Resolve the effective plan for a user:
/// - Checks personal subscription
/// - Checks all workspace memberships with active subscriptions
/// - Returns the highest plan (exclusive > enterprise > freemium)
pub async fn resolve_effective_plan(pool: &DbPool, user_id: uuid::Uuid) -> String {
    let personal_plan = repository::get_personal_plan(pool, user_id)
        .await
        .ok()
        .flatten();

    let workspace_plan = repository::get_highest_workspace_plan(pool, user_id)
        .await
        .ok()
        .flatten();

    // Priority: exclusive > enterprise > freemium
    let priority = |plan: &str| -> u8 {
        match plan {
            "exclusive" => 3,
            "enterprise" => 2,
            _ => 1,
        }
    };

    match (personal_plan, workspace_plan) {
        (Some(p), Some(w)) => {
            if priority(&p) >= priority(&w) { p } else { w }
        }
        (Some(p), None) => p,
        (None, Some(w)) => w,
        (None, None) => "freemium".to_string(),
    }
}

pub fn generate_jwt_token(user: &User, secret: &str, plan: &str) -> ServiceResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
        role: user.role.clone(),
        plan: plan.to_string(),
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
