//! Domain management models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Custom domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDomain {
    /// Unique domain ID
    pub id: Uuid,
    /// Project ID
    pub project_id: Uuid,
    /// Domain name
    pub domain: String,
    /// Whether this is the primary domain
    pub is_primary: bool,
    /// SSL certificate status
    pub ssl_status: SslStatus,
    /// DNS verification status
    pub dns_status: DnsStatus,
    /// DNS records required
    pub dns_records: Vec<DnsRecord>,
    /// When the domain was added
    pub created_at: DateTime<Utc>,
    /// When the domain was verified
    pub verified_at: Option<DateTime<Utc>>,
}

impl CustomDomain {
    /// Check if the domain is ready to serve traffic
    pub fn is_ready(&self) -> bool {
        self.ssl_status == SslStatus::Active && self.dns_status == DnsStatus::Verified
    }
}

/// SSL certificate status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SslStatus {
    /// Certificate pending
    Pending,
    /// Certificate issued and active
    Active,
    /// Certificate renewal pending
    Renewing,
    /// Certificate failed
    Failed,
}

/// DNS verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DnsStatus {
    /// DNS not configured
    NotConfigured,
    /// DNS verification pending
    Pending,
    /// DNS verified
    Verified,
    /// DNS verification failed
    Failed,
}

/// DNS record configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    /// Record type (A, CNAME, TXT)
    pub record_type: String,
    /// Record name/host
    pub name: String,
    /// Record value
    pub value: String,
    /// TTL in seconds
    pub ttl: u32,
    /// Whether this record is configured
    pub is_configured: bool,
}

/// Request to add a custom domain
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddDomainRequest {
    /// Domain name
    #[validate(length(min = 1, max = 253, message = "Domain must be 1-253 characters"))]
    pub domain: String,
}

/// Request to set primary domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPrimaryRequest {
    /// Domain ID to set as primary
    pub domain_id: Uuid,
}

/// Domain list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainListResponse {
    pub domains: Vec<CustomDomain>,
    pub total: u32,
    pub max_domains: u32,
}

/// DNS verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsVerificationResponse {
    pub domain: String,
    pub status: DnsStatus,
    pub message: String,
    pub next_check_in_seconds: Option<u64>,
}

/// SSL certificate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslStatusResponse {
    pub domain: String,
    pub status: SslStatus,
    pub issuer: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_domain_is_ready() {
        let mut domain = CustomDomain {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            domain: "example.com".to_string(),
            is_primary: true,
            ssl_status: SslStatus::Active,
            dns_status: DnsStatus::Verified,
            dns_records: vec![],
            created_at: Utc::now(),
            verified_at: None,
        };

        assert!(domain.is_ready());

        domain.ssl_status = SslStatus::Pending;
        assert!(!domain.is_ready());
    }

    #[test]
    fn test_add_domain_validation() {
        let request = AddDomainRequest {
            domain: "example.com".to_string(),
        };

        assert!(request.validate().is_ok());

        let invalid = AddDomainRequest {
            domain: "".to_string(),
        };

        assert!(invalid.validate().is_err());
    }
}
