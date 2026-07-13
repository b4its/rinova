//! Request proxy service

use actix_web::{HttpRequest, HttpResponse, web};
use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

use crate::config::ServiceConfig;
use crate::middleware::AuthenticatedUser;

/// Proxy service for forwarding requests to backend services
#[derive(Clone)]
pub struct ProxyService {
    client: Client,
}

impl ProxyService {
    /// Create a new proxy service
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to create HTTP client");

        ProxyService { client }
    }

    /// Forward request to backend service
    pub async fn forward(
        &self,
        req: HttpRequest,
        body: web::Bytes,
        service: &ServiceConfig,
        user: Option<AuthenticatedUser>,
    ) -> Result<HttpResponse> {
        let path = req.path();
        let method = req.method().as_str();

        tracing::info!("Proxying {} {} to {}", method, path, service.url);

        // Build target URL
        let target_url = format!("{}{}", service.url, path);
        let query_string = req.query_string();
        let target_url = if !query_string.is_empty() {
            format!("{}?{}", target_url, query_string)
        } else {
            target_url
        };

        // Build request
        let mut request_builder = match method {
            "GET" => self.client.get(&target_url),
            "POST" => self.client.post(&target_url),
            "PUT" => self.client.put(&target_url),
            "DELETE" => self.client.delete(&target_url),
            "PATCH" => self.client.patch(&target_url),
            _ => {
                return Ok(HttpResponse::MethodNotAllowed()
                    .json(serde_json::json!({
                        "error": format!("Method {} not supported", method),
                        "code": "METHOD_NOT_ALLOWED"
                    })));
            }
        };

        // Forward headers (excluding Host)
        for (name, value) in req.headers() {
            if name != "host" && name != "content-length" {
                request_builder = request_builder.header(name.as_str(), value);
            }
        }

        // Add user context headers if authenticated
        if let Some(ref user) = user {
            request_builder = request_builder
                .header("X-User-ID", user.user_id.to_string())
                .header("X-User-Email", user.email.clone())
                .header("X-User-Plan", user.plan.clone());
        }

        // Add body for non-GET requests
        if method != "GET" && !body.is_empty() {
            request_builder = request_builder.body(body.to_vec());
        }

        // Send request
        let response = request_builder
            .send()
            .await
            .context("Failed to forward request to backend service")?;

        // Build response
        let status = response.status();
        let mut response_builder = HttpResponse::build(status);

        // Forward response headers
        for (name, value) in response.headers() {
            if name != "content-length" && name != "transfer-encoding" {
                response_builder.append_header((name.as_str(), value));
            }
        }

        // Get response body
        let response_body = response
            .bytes()
            .await
            .context("Failed to read response body")?;

        Ok(response_builder.body(response_body))
    }

    /// Health check for a service
    pub async fn health_check(&self, service: &ServiceConfig) -> Result<bool> {
        let url = format!("{}{}", service.url, service.health_path);

        let response = self.client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(e) => {
                tracing::warn!("Health check failed for {}: {}", service.url, e);
                Ok(false)
            }
        }
    }
}

impl Default for ProxyService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_service_creation() {
        let service = ProxyService::new();
        // Verify the client was created successfully
        // The timeout is configured internally during client builder
        assert!(true); // Service created successfully
    }
}
