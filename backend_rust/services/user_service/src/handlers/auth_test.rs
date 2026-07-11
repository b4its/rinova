#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::health_check;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new().service(health_check)
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "user_service");
    }
}

#[cfg(test)]
mod validation_tests {
    #[test]
    fn test_password_validation() {
        // Valid passwords
        assert!(shared::utils::validate_password("Password123"));
        assert!(shared::utils::validate_password("Abcdefg1"));
        assert!(shared::utils::validate_password("UPPERlower123"));

        // Invalid passwords - too short
        assert!(!shared::utils::validate_password("Pass1"));

        // Invalid passwords - no uppercase
        assert!(!shared::utils::validate_password("password123"));

        // Invalid passwords - no lowercase
        assert!(!shared::utils::validate_password("PASSWORD123"));

        // Invalid passwords - no numbers
        assert!(!shared::utils::validate_password("PasswordPassword"));
    }

    #[test]
    fn test_email_validation() {
        // Valid emails (RFC 5322)
        assert!(shared::utils::validate_email("test@example.com"));
        assert!(shared::utils::validate_email("user.name@example.com"));
        assert!(shared::utils::validate_email("user+tag@example.com"));
        assert!(shared::utils::validate_email("user@subdomain.example.com"));

        // Invalid emails
        assert!(!shared::utils::validate_email("invalid-email"));
        assert!(!shared::utils::validate_email("user@"));
        assert!(!shared::utils::validate_email("@example.com"));
        assert!(!shared::utils::validate_email("user@example"));
    }

    #[test]
    fn test_bcrypt_cost_factor_12() {
        use bcrypt::{hash, verify};

        let password = "Password123";
        let hash_result = hash(password, 12);
        
        assert!(hash_result.is_ok());
        
        let hashed = hash_result.unwrap();
        assert!(verify(password, &hashed).unwrap());
        assert!(!verify("wrongpassword", &hashed).unwrap());
    }

    #[test]
    fn test_register_request_validation_valid_email() {
        use crate::handlers::RegisterRequest;
        use validator::Validate;

        let valid_req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "Password123".to_string(),
            account_type: None,
        };
        assert!(valid_req.validate().is_ok());
    }

    #[test]
    fn test_register_request_validation_invalid_email() {
        use crate::handlers::RegisterRequest;
        use validator::Validate;

        let invalid_req = RegisterRequest {
            email: "invalid-email".to_string(),
            password: "Password123".to_string(),
            account_type: None,
        };
        assert!(invalid_req.validate().is_err());
    }

    #[test]
    fn test_register_request_validation_password_too_short() {
        use crate::handlers::RegisterRequest;
        use validator::Validate;

        let req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "Pass1".to_string(),  // Too short
            account_type: None,
        };
        assert!(req.validate().is_err());
    }
}

#[cfg(test)]
mod email_verification_tests {
    use crate::services::{generate_verification_token, verify_verification_token};
    use shared::errors::ServiceError;

    const TEST_SECRET: &str = "test-secret-key-for-verification";

    #[test]
    fn test_generate_verification_token_success() {
        let user_id = uuid::Uuid::new_v4().to_string();
        let email = "test@example.com";

        let token = generate_verification_token(&user_id, email, TEST_SECRET);
        assert!(token.is_ok());
        
        let token_str = token.unwrap();
        assert!(!token_str.is_empty());
    }

    #[test]
    fn test_verify_valid_token_success() {
        let user_id = uuid::Uuid::new_v4().to_string();
        let email = "test@example.com";

        let token = generate_verification_token(&user_id, email, TEST_SECRET).unwrap();
        let claims = verify_verification_token(&token, TEST_SECRET);

        assert!(claims.is_ok());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.purpose, "email_verification");
    }

    #[test]
    fn test_verify_token_wrong_secret_fails() {
        let user_id = uuid::Uuid::new_v4().to_string();
        let email = "test@example.com";

        let token = generate_verification_token(&user_id, email, TEST_SECRET).unwrap();
        let result = verify_verification_token(&token, "wrong-secret");

        assert!(result.is_err());
        // Should return TokenInvalid for wrong secret
        match result {
            Err(ServiceError::TokenInvalid) => (),
            _ => panic!("Expected TokenInvalid error"),
        }
    }

    #[test]
    fn test_verify_invalid_token_fails() {
        let result = verify_verification_token("invalid-token", TEST_SECRET);
        assert!(result.is_err());
        match result {
            Err(ServiceError::TokenInvalid) => (),
            _ => panic!("Expected TokenInvalid error"),
        }
    }

    #[test]
    fn test_token_contains_correct_purpose() {
        let user_id = uuid::Uuid::new_v4().to_string();
        let email = "test@example.com";

        let token = generate_verification_token(&user_id, email, TEST_SECRET).unwrap();
        
        // Verify and check the purpose field in claims
        let claims = verify_verification_token(&token, TEST_SECRET).unwrap();
        
        assert_eq!(claims.purpose, "email_verification");
    }

    #[test]
    fn test_expired_token_returns_correct_error() {
        use jsonwebtoken::{encode, Header, EncodingKey};
        use chrono::{Duration, Utc};
        use serde::Serialize;

        #[derive(Serialize)]
        struct TestClaims {
            sub: String,
            email: String,
            exp: usize,
            iat: usize,
            purpose: String,
        }

        // Create an already expired token (expired 1 hour ago)
        let expiration = Utc::now()
            .checked_sub_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = TestClaims {
            sub: uuid::Uuid::new_v4().to_string(),
            email: "test@example.com".to_string(),
            exp: expiration,
            iat: Utc::now().timestamp() as usize,
            purpose: "email_verification".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(TEST_SECRET.as_ref()),
        ).unwrap();

        let result = verify_verification_token(&token, TEST_SECRET);
        assert!(result.is_err());
        
        // Should return TokenExpired for expired token
        match result {
            Err(ServiceError::TokenExpired) => (),
            _ => panic!("Expected TokenExpired error, got {:?}", result),
        }
    }
}

#[cfg(test)]
mod resend_verification_tests {
    use crate::handlers::ResendVerificationRequest;

    #[test]
    fn test_resend_verification_request_deserialize() {
        let json = r#"{"email": "test@example.com"}"#;
        let req: ResendVerificationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
    }
}
