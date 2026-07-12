//! JWT authentication middleware

use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, http::StatusCode};
use chrono::{Duration, Utc};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at
    pub iat: i64,
    /// Expiration time
    pub exp: i64,
    /// User's subscription plan
    pub plan: String,
    /// User's email
    pub email: String,
}

/// Authenticated user extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub plan: String,
    pub email: String,
}

impl AuthenticatedUser {
    /// Extract user from request
    pub fn extract(req: &HttpRequest, jwt_secret: &str) -> Result<Self, AuthError> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Check Bearer prefix
        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::InvalidToken);
        }

        let token = &auth_header[7..];

        // Decode and validate token
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?
        .claims;

        // Parse user ID
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthenticatedUser {
            user_id,
            plan: claims.plan,
            email: claims.email,
        })
    }

    /// Check if user has a specific plan or higher
    pub fn has_plan(&self, required_plan: &str) -> bool {
        match (self.plan.as_str(), required_plan) {
            ("exclusive", _) => true,
            ("enterprise", "enterprise") | ("enterprise", "freemium") => true,
            ("freemium", "freemium") => true,
            _ => false,
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get JWT secret from app data
        let jwt_secret = req
            .app_data::<actix_web::web::Data<String>>()
            .map(|d| d.as_ref().clone())
            .unwrap_or_default();

        match Self::extract(req, &jwt_secret) {
            Ok(user) => ready(Ok(user)),
            Err(e) => ready(Err(e.into())),
        }
    }
}

/// Authentication errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authentication token")]
    MissingToken,
    #[error("Invalid authentication token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
}

impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        let message = err.to_string();
        let code = match &err {
            AuthError::MissingToken => "MISSING_TOKEN",
            AuthError::InvalidToken => "INVALID_TOKEN",
            AuthError::TokenExpired => "TOKEN_EXPIRED",
        };

        actix_web::error::InternalError::from_response(
            err,
            actix_web::HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": message,
                    "code": code
                }))
                .into(),
        )
        .into()
    }
}

/// JWT service for token generation
pub struct JwtService {
    secret: String,
    expiry_hours: u64,
}

impl JwtService {
    /// Create a new JWT service
    pub fn new(secret: String, expiry_hours: u64) -> Self {
        JwtService { secret, expiry_hours }
    }

    /// Generate a new JWT token
    pub fn generate_token(&self, user_id: Uuid, email: &str, plan: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiry_hours as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            plan: plan.to_string(),
            email: email.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    /// Validate a JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|e| {
            if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                AuthError::TokenExpired
            } else {
                AuthError::InvalidToken
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let service = JwtService::new("test-secret".to_string(), 168);
        let user_id = Uuid::new_v4();

        let token = service.generate_token(user_id, "test@example.com", "enterprise").unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.plan, "enterprise");
    }

    #[test]
    fn test_authenticated_user_has_plan() {
        let user = AuthenticatedUser {
            user_id: Uuid::new_v4(),
            plan: "enterprise".to_string(),
            email: "test@example.com".to_string(),
        };

        assert!(user.has_plan("freemium"));
        assert!(user.has_plan("enterprise"));
        assert!(!user.has_plan("exclusive"));
    }
}
