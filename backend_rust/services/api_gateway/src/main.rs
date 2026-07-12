//! API Gateway Service
//!
//! Central gateway for all microservices with:
//! - JWT authentication
//! - Rate limiting per subscription plan
//! - Request routing and proxying
//! - Request/response transformation
//!
//! ## Routes
//!
//! - /api/v1/auth/*          → User Service
//! - /api/v1/users/*         → User Service
//! - /api/v1/workspaces/*    → Workspace Service
//! - /api/v1/subscriptions/* → Subscription Service
//! - /api/v1/projects/*      → Project Service
//! - /api/v1/publish/*       → Publishing Service
//! - /api/v1/blockchain/*    → Blockchain Service

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer, HttpRequest, HttpResponse};
use dotenv::dotenv;
use std::sync::Arc;

mod config;
mod middleware;
mod proxy;

use config::GatewayConfig;
use middleware::{AuthenticatedUser, RateLimitConfig, RateLimitMiddleware};
use proxy::ProxyService;

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    services: std::collections::HashMap<String, bool>,
}

/// Health check endpoint
async fn health(
    config: web::Data<GatewayConfig>,
    proxy: web::Data<ProxyService>,
) -> web::Json<HealthResponse> {
    let mut services_status = std::collections::HashMap::new();

    // Check health of all services
    for (name, service_config) in &config.services {
        let healthy = proxy.health_check(service_config).await.unwrap_or(false);
        services_status.insert(name.clone(), healthy);
    }

    let overall_status = if services_status.values().all(|&v| v) {
        "healthy"
    } else {
        "degraded"
    };

    web::Json(HealthResponse {
        status: overall_status.to_string(),
        service: "api_gateway".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services: services_status,
    })
}

/// Main proxy handler for all requests
async fn proxy_handler(
    req: HttpRequest,
    body: web::Bytes,
    config: web::Data<GatewayConfig>,
    proxy: web::Data<ProxyService>,
    rate_limiter: web::Data<RateLimitMiddleware>,
    jwt_secret: web::Data<String>,
) -> HttpResponse {
    let path = req.path();

    // Skip auth for health endpoint
    if path == "/health" {
        return health(config, proxy).await.into();
    }

    // Skip auth for certain endpoints
    let skip_auth = path.starts_with("/api/v1/auth/register")
        || path.starts_with("/api/v1/auth/login")
        || path.starts_with("/api/v1/auth/verify-email")
        || path.starts_with("/api/v1/subscriptions/plans");

    // Authenticate user if required
    let user = if !skip_auth {
        match AuthenticatedUser::extract(&req, &jwt_secret) {
            Ok(user) => Some(user),
            Err(e) => {
                tracing::warn!("Authentication failed: {}", e);
                return HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "error": "Authentication required",
                        "code": "UNAUTHORIZED"
                    }));
            }
        }
    } else {
        None
    };

    // Check rate limit
    if let Some(ref user) = user {
        if let Err(_) = rate_limiter.check(&user.user_id.to_string(), &user.plan).await {
            return HttpResponse::TooManyRequests()
                .insert_header(("Retry-After", "60"))
                .json(serde_json::json!({
                    "error": "Rate limit exceeded. Please try again later.",
                    "code": "RATE_LIMIT_EXCEEDED",
                    "retry_after_seconds": 60
                }));
        }
    }

    // Get target service
    let (service_name, service_config) = match config.get_service_url(path) {
        Some((name, config)) => (name, config),
        None => {
            return HttpResponse::NotFound()
                .json(serde_json::json!({
                    "error": "Service not found",
                    "code": "SERVICE_NOT_FOUND"
                }));
        }
    };

    tracing::debug!(
        "Routing {} {} to {} service",
        req.method(),
        path,
        service_name
    );

    // Forward request
    match proxy.forward(req, body, service_config, user).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Proxy error: {}", e);
            HttpResponse::BadGateway()
                .json(serde_json::json!({
                    "error": "Service temporarily unavailable",
                    "code": "SERVICE_UNAVAILABLE"
                }))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    // Load configuration
    let config = GatewayConfig::from_env()
        .expect("Failed to load gateway configuration");

    let port = config.server.port;
    tracing::info!("Starting API Gateway on port {}", port);

    // Initialize services
    let proxy = ProxyService::new();
    let rate_limit_config = RateLimitConfig::default();
    let rate_limiter = RateLimitMiddleware::new(rate_limit_config);
    let jwt_secret = config.jwt.secret.clone();

    // Start server
    let bind_address = format!("{}:{}", config.server.host, port);
    tracing::info!("Starting HTTP server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(proxy.clone()))
            .app_data(web::Data::new(rate_limiter.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .route("/health", web::get().to(health))
            // Catch-all route for proxying
            .default_service(web::route().to(proxy_handler))
    })
    .bind(&bind_address)?
    .run()
    .await
}
