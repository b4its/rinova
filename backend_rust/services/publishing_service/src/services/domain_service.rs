//! Domain service for custom domain management

use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    AddDomainRequest, CustomDomain, DnsRecord, DnsStatus, DnsVerificationResponse,
    DomainListResponse, SslStatus, SslStatusResponse,
};

/// Domain service for custom domain management
#[derive(Clone)]
pub struct DomainService {
    db: Option<PgPool>,
    max_domains_per_project: u32,
}

impl DomainService {
    /// Create a new domain service
    pub fn new() -> Self {
        DomainService {
            db: None,
            max_domains_per_project: 5,
        }
    }

    /// Create with database pool
    pub fn with_db(db: PgPool) -> Self {
        DomainService {
            db: Some(db),
            max_domains_per_project: 5,
        }
    }

    /// List custom domains for a project
    pub async fn list_domains(&self, project_id: Uuid) -> Result<DomainListResponse> {
        tracing::info!("Listing domains for project: {}", project_id);

        // Note: Would query from database
        let domains: Vec<CustomDomain> = vec![];

        Ok(DomainListResponse {
            total: domains.len() as u32,
            max_domains: self.max_domains_per_project,
            domains,
        })
    }

    /// Add a custom domain to a project
    pub async fn add_domain(&self, project_id: Uuid, request: AddDomainRequest) -> Result<CustomDomain> {
        tracing::info!("Adding domain {} to project {}", request.domain, project_id);

        // Validate domain format
        request.validate().context("Invalid domain")?;

        // Check domain count limit
        let current_count = self.count_domains(project_id).await?;
        if current_count >= self.max_domains_per_project {
            anyhow::bail!(
                "Maximum number of domains ({}) reached for this project",
                self.max_domains_per_project
            );
        }

        // Validate domain format (basic check)
        if !self.is_valid_domain(&request.domain) {
            anyhow::bail!("Invalid domain format");
        }

        // Generate DNS records
        let dns_records = self.generate_dns_records(&request.domain);

        let domain = CustomDomain {
            id: Uuid::new_v4(),
            project_id,
            domain: request.domain.clone(),
            is_primary: current_count == 0,
            ssl_status: SslStatus::Pending,
            dns_status: DnsStatus::NotConfigured,
            dns_records,
            created_at: chrono::Utc::now(),
            verified_at: None,
        };

        // Note: Would save to database

        tracing::info!("Domain {} added to project {}", request.domain, project_id);

        Ok(domain)
    }

    /// Remove a custom domain
    pub async fn remove_domain(&self, project_id: Uuid, domain_id: Uuid) -> Result<()> {
        tracing::info!("Removing domain {} from project {}", domain_id, project_id);

        // Note: Would verify ownership and delete from database

        Ok(())
    }

    /// Set primary domain
    pub async fn set_primary(&self, project_id: Uuid, domain_id: Uuid) -> Result<CustomDomain> {
        tracing::info!("Setting domain {} as primary for project {}", domain_id, project_id);

        // Note: Would update database to set primary flag

        let domain = CustomDomain {
            id: domain_id,
            project_id,
            domain: "example.com".to_string(),
            is_primary: true,
            ssl_status: SslStatus::Active,
            dns_status: DnsStatus::Verified,
            dns_records: vec![],
            created_at: chrono::Utc::now(),
            verified_at: None,
        };

        Ok(domain)
    }

    /// Verify DNS configuration
    pub async fn verify_dns(&self, project_id: Uuid, domain: &str) -> Result<DnsVerificationResponse> {
        tracing::info!("Verifying DNS for domain: {}", domain);

        // Note: Would perform actual DNS lookups
        // For now, return pending status

        Ok(DnsVerificationResponse {
            domain: domain.to_string(),
            status: DnsStatus::Pending,
            message: "DNS verification in progress. This may take up to 48 hours.".to_string(),
            next_check_in_seconds: Some(300),
        })
    }

    /// Get SSL certificate status
    pub async fn get_ssl_status(&self, project_id: Uuid, domain: &str) -> Result<SslStatusResponse> {
        tracing::info!("Getting SSL status for domain: {}", domain);

        // Note: Would query certificate status from Let's Encrypt or storage

        Ok(SslStatusResponse {
            domain: domain.to_string(),
            status: SslStatus::Pending,
            issuer: None,
            valid_from: None,
            valid_until: None,
        })
    }

    /// Count domains for a project
    async fn count_domains(&self, project_id: Uuid) -> Result<u32> {
        // Note: Would query from database
        Ok(0)
    }

    /// Validate domain format
    fn is_valid_domain(&self, domain: &str) -> bool {
        // Basic domain validation
        let domain_regex = regex::Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+$").unwrap();
        domain_regex.is_match(domain) && domain.len() <= 253
    }

    /// Generate required DNS records for a domain
    fn generate_dns_records(&self, domain: &str) -> Vec<DnsRecord> {
        vec![
            DnsRecord {
                record_type: "A".to_string(),
                name: "@".to_string(),
                value: "192.0.2.1".to_string(), // Would be actual IP
                ttl: 3600,
                is_configured: false,
            },
            DnsRecord {
                record_type: "CNAME".to_string(),
                name: "www".to_string(),
                value: format!("{}.rinova.app", domain),
                ttl: 3600,
                is_configured: false,
            },
            DnsRecord {
                record_type: "TXT".to_string(),
                name: "_rinova-verification".to_string(),
                value: format!("rinova-site-verification={}", Uuid::new_v4()),
                ttl: 3600,
                is_configured: false,
            },
        ]
    }
}

impl Default for DomainService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_domain() {
        let service = DomainService::new();

        assert!(service.is_valid_domain("example.com"));
        assert!(service.is_valid_domain("sub.example.com"));
        assert!(service.is_valid_domain("my-site.example.co.uk"));
        assert!(!service.is_valid_domain("invalid"));
        assert!(!service.is_valid_domain("-invalid.com"));
    }

    #[test]
    fn test_generate_dns_records() {
        let service = DomainService::new();
        let records = service.generate_dns_records("example.com");

        assert!(!records.is_empty());
        assert!(records.iter().any(|r| r.record_type == "A"));
        assert!(records.iter().any(|r| r.record_type == "CNAME"));
        assert!(records.iter().any(|r| r.record_type == "TXT"));
    }

    #[tokio::test]
    async fn test_add_domain() {
        let service = DomainService::new();
        let request = AddDomainRequest {
            domain: "example.com".to_string(),
        };

        let result = service.add_domain(Uuid::new_v4(), request).await;
        assert!(result.is_ok());
    }
}
