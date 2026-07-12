//! Audit trail models for publish records

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publish audit record from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    /// Project ID
    pub project_id: String,
    /// SHA-256 hash of the website bundle
    pub content_hash: String,
    /// Timestamp when published (Unix epoch milliseconds)
    pub timestamp: i64,
    /// Publisher Ethereum address
    pub publisher_address: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// Optional IPFS CID
    pub ipfs_cid: Option<String>,
    /// Published URL
    pub published_url: String,
}

impl AuditRecord {
    /// Get the timestamp as DateTime
    pub fn timestamp_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.timestamp).unwrap_or_else(|| Utc::now())
    }
}

/// Request to record a publish event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPublishRequest {
    /// Project ID
    pub project_id: String,
    /// Content hash (will be computed if not provided)
    pub content_hash: Option<String>,
    /// Content bundle (for hash computation)
    pub content_bundle: Option<Vec<u8>>,
    /// Optional IPFS CID
    pub ipfs_cid: Option<String>,
    /// Published URL
    pub published_url: String,
}

/// Audit trail verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditVerification {
    /// Project ID
    pub project_id: String,
    /// Content hash being verified
    pub content_hash: String,
    /// Whether the hash matches
    pub verified: bool,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: i64,
}

/// Publish history response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishHistory {
    pub project_id: String,
    pub records: Vec<AuditRecord>,
    pub total: u64,
}

/// Hash computation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResult {
    /// SHA-256 hash (hex encoded)
    pub hash: String,
    /// Time taken to compute (milliseconds)
    pub computation_time_ms: u64,
    /// Content size (bytes)
    pub content_size: u64,
}

impl HashResult {
    /// Create a new hash result
    pub fn new(hash: String, computation_time_ms: u64, content_size: u64) -> Self {
        HashResult {
            hash,
            computation_time_ms,
            content_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_record_timestamp() {
        let record = AuditRecord {
            project_id: "project-123".to_string(),
            content_hash: "abc123".to_string(),
            timestamp: 1609459200000,
            publisher_address: "0xabcdef".to_string(),
            tx_hash: "0xxyz".to_string(),
            block_number: 12345,
            ipfs_cid: None,
            published_url: "https://example.com".to_string(),
        };

        let dt = record.timestamp_datetime();
        assert_eq!(dt.year(), 2021);
    }
}
