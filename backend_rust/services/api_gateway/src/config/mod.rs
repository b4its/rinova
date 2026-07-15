//! API Gateway configuration

use serde::Deserialize;
use std::collections::HashMap;

/// Gateway configuration
#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    /// Server settings
    pub server: ServerConfig,
    /// JWT settings
    pub jwt: JwtConfig,
    /// Rate limiting settings
    pub rate_limit: RateLimitConfig,
    /// Service routes
    pub services: HashMap<String, ServiceConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_jwt_expiry")]
    pub expiry_hours: u64,
}

fn default_jwt_expiry() -> u64 {
    168 // 7 days
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Redis URL for distributed rate limiting
    pub redis_url: Option<String>,
    /// Default rate limit per minute
    #[serde(default = "default_rate_limit")]
    pub default_limit: u32,
}

fn default_true() -> bool {
    true
}

fn default_rate_limit() -> u32 {
    100
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    /// Service URL
    pub url: String,
    /// Health check path
    #[serde(default = "default_health_path")]
    pub health_path: String,
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_health_path() -> String {
    "/health".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl GatewayConfig {
    /// Load configuration from environment
    pub fn from_env() -> anyhow::Result<Self> {
        let mut services = HashMap::new();

        // Service URLs from environment
        services.insert(
            "users".to_string(),
            ServiceConfig {
                url: std::env::var("USER_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8001".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 30,
            },
        );

        services.insert(
            "workspaces".to_string(),
            ServiceConfig {
                url: std::env::var("WORKSPACE_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8007".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 30,
            },
        );

        services.insert(
            "subscriptions".to_string(),
            ServiceConfig {
                url: std::env::var("SUBSCRIPTION_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8002".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 30,
            },
        );

        services.insert(
            "projects".to_string(),
            ServiceConfig {
                url: std::env::var("PROJECT_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8003".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 30,
            },
        );

        services.insert(
            "publish".to_string(),
            ServiceConfig {
                url: std::env::var("PUBLISHING_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8004".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 60, // Longer timeout for publish operations
            },
        );

        services.insert(
            "blockchain".to_string(),
            ServiceConfig {
                url: std::env::var("BLOCKCHAIN_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8005".to_string()),
                health_path: "/health".to_string(),
                timeout_secs: 60, // Longer timeout for blockchain ops
            },
        );

        Ok(GatewayConfig {
            server: ServerConfig {
                host: std::env::var("GATEWAY_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("GATEWAY_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
            },
            jwt: JwtConfig {
                secret: std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                expiry_hours: 168,
            },
            rate_limit: RateLimitConfig {
                enabled: true,
                redis_url: std::env::var("REDIS_URL").ok(),
                default_limit: 100,
            },
            services,
        })
    }

    /// Get service URL by route prefix
    pub fn get_service_url(&self, path: &str) -> Option<(&str, &ServiceConfig)> {
        // Match path prefix to service
        if path.starts_with("/api/v1/auth") || path.starts_with("/api/v1/users") || path.starts_with("/api/v1/admin") {
            self.services.get("users").map(|s| ("users", s))
        } else if path.starts_with("/api/v1/workspaces") {
            self.services.get("workspaces").map(|s| ("workspaces", s))
        } else if path.starts_with("/api/v1/subscriptions") {
            self.services.get("subscriptions").map(|s| ("subscriptions", s))
        } else if path.starts_with("/api/v1/projects") {
            self.services.get("projects").map(|s| ("projects", s))
        } else if path.starts_with("/api/v1/publish") {
            self.services.get("publish").map(|s| ("publish", s))
        } else if path.starts_with("/api/v1/blockchain") {
            self.services.get("blockchain").map(|s| ("blockchain", s))
        } else {
            None
        }
    }
}
