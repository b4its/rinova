//! Rate limiting middleware

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use actix_web::{dev::ServiceRequest, Error, HttpResponse};
use tokio::sync::RwLock;

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Default limit per minute
    pub default_limit: u32,
    /// Plan-specific limits
    pub plan_limits: HashMap<String, u32>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        let mut plan_limits = HashMap::new();
        plan_limits.insert("freemium".to_string(), 100);
        plan_limits.insert("enterprise".to_string(), 1000);
        plan_limits.insert("exclusive".to_string(), 5000);

        RateLimitConfig {
            default_limit: 100,
            plan_limits,
        }
    }
}

impl RateLimitConfig {
    /// Get rate limit for a plan
    pub fn get_limit(&self, plan: &str) -> u32 {
        self.plan_limits.get(plan).copied().unwrap_or(self.default_limit)
    }
}

/// In-memory rate limit store
#[derive(Debug, Clone)]
pub struct RateLimitStore {
    limits: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    config: RateLimitConfig,
}

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    reset_at: Instant,
}

impl RateLimitStore {
    /// Create a new rate limit store
    pub fn new(config: RateLimitConfig) -> Self {
        RateLimitStore {
            limits: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check and increment rate limit
    pub async fn check_rate_limit(&self, key: &str, plan: &str) -> RateLimitResult {
        let limit = self.config.get_limit(plan);
        let now = Instant::now();

        let mut limits = self.limits.write().await;

        // Clean up expired entries
        limits.retain(|_, entry| entry.reset_at > now);

        let entry = limits.entry(key.to_string()).or_insert(RateLimitEntry {
            count: 0,
            reset_at: now + Duration::from_secs(60),
        });

        // Reset if window expired
        if entry.reset_at < now {
            entry.count = 0;
            entry.reset_at = now + Duration::from_secs(60);
        }

        // Check limit
        if entry.count >= limit {
            let remaining_seconds = (entry.reset_at - now).as_secs();
            return RateLimitResult::Exceeded {
                limit,
                remaining_seconds,
            };
        }

        // Increment counter
        entry.count += 1;

        RateLimitResult::Allowed {
            limit,
            remaining: limit - entry.count,
        }
    }
}

/// Rate limit check result
#[derive(Debug, Clone)]
pub enum RateLimitResult {
    Allowed {
        limit: u32,
        remaining: u32,
    },
    Exceeded {
        limit: u32,
        remaining_seconds: u64,
    },
}

/// Rate limit middleware
pub struct RateLimitMiddleware {
    store: RateLimitStore,
}

impl RateLimitMiddleware {
    /// Create a new rate limit middleware
    pub fn new(config: RateLimitConfig) -> Self {
        RateLimitMiddleware {
            store: RateLimitStore::new(config),
        }
    }

    /// Check rate limit for a request
    pub async fn check(&self, user_id: &str, plan: &str) -> Result<(), Error> {
        match self.store.check_rate_limit(user_id, plan).await {
            RateLimitResult::Allowed { limit, remaining } => {
                // Add rate limit headers to response
                // Note: This would be done via response headers in a real implementation
                tracing::debug!(
                    "Rate limit OK: user={}, limit={}, remaining={}",
                    user_id, limit, remaining
                );
                Ok(())
            }
            RateLimitResult::Exceeded {
                limit,
                remaining_seconds,
            } => {
                tracing::warn!(
                    "Rate limit exceeded: user={}, limit={}, retry_after={}s",
                    user_id, limit, remaining_seconds
                );
                Err(actix_web::error::InternalError::from_response(
                    "Rate limit exceeded",
                    HttpResponse::TooManyRequests()
                        .insert_header(("Retry-After", "60"))
                        .json(serde_json::json!({
                            "error": "Rate limit exceeded. Please try again later.",
                            "code": "RATE_LIMIT_EXCEEDED",
                            "retry_after_seconds": remaining_seconds
                        }))
                        .into(),
                )
                .into())
            }
        }
    }
}

/// Rate limit based on user's subscription plan
/// - Freemium: 100 requests/minute
/// - Enterprise: 1000 requests/minute
/// - Exclusive: 5000 requests/minute
pub fn get_rate_limit_for_plan(plan: &str) -> u32 {
    match plan {
        "freemium" => 100,
        "enterprise" => 1000,
        "exclusive" => 5000,
        _ => 100,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_store() {
        let config = RateLimitConfig::default();
        let store = RateLimitStore::new(config);

        // First request should be allowed
        match store.check_rate_limit("user1", "freemium").await {
            RateLimitResult::Allowed { limit, remaining } => {
                assert_eq!(limit, 100);
                assert_eq!(remaining, 99);
            }
            _ => panic!("Expected Allowed"),
        }

        // Multiple requests
        for i in 0..99 {
            let result = store.check_rate_limit("user1", "freemium").await;
            match result {
                RateLimitResult::Allowed { remaining, .. } => {
                    assert_eq!(remaining, 98 - i);
                }
                _ => panic!("Expected Allowed at iteration {}", i),
            }
        }

        // Should be exceeded after 100 requests
        match store.check_rate_limit("user1", "freemium").await {
            RateLimitResult::Exceeded { limit, .. } => {
                assert_eq!(limit, 100);
            }
            _ => panic!("Expected Exceeded"),
        }
    }

    #[test]
    fn test_rate_limit_config() {
        let config = RateLimitConfig::default();

        assert_eq!(config.get_limit("freemium"), 100);
        assert_eq!(config.get_limit("enterprise"), 1000);
        assert_eq!(config.get_limit("exclusive"), 5000);
        assert_eq!(config.get_limit("unknown"), 100);
    }
}
