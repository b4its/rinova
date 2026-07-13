//! Asset ownership models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Asset ownership record from blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipRecord {
    /// Unique asset ID
    pub asset_id: String,
    /// Owner Ethereum address
    pub owner_address: String,
    /// Timestamp when ownership was recorded (Unix epoch milliseconds)
    pub timestamp: i64,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// SHA-256 hash of the asset
    pub asset_hash: String,
    /// Optional IPFS CID
    pub ipfs_cid: Option<String>,
}

impl OwnershipRecord {
    /// Get the timestamp as DateTime
    pub fn timestamp_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.timestamp).unwrap_or_else(|| Utc::now())
    }
}

/// Request to record ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordOwnershipRequest {
    /// Asset ID (alphanumeric, max 64 chars)
    pub asset_id: String,
    /// Owner Ethereum address
    pub owner_address: String,
    /// SHA-256 hash of the asset
    pub asset_hash: String,
    /// Optional IPFS CID
    pub ipfs_cid: Option<String>,
}

/// Request to transfer ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnershipRequest {
    /// Asset ID
    pub asset_id: String,
    /// New owner Ethereum address
    pub new_owner_address: String,
    /// Digital signature from current owner
    pub signature: String,
}

/// Ownership verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipProof {
    /// Asset ID
    pub asset_id: String,
    /// Current owner address
    pub owner_address: String,
    /// Timestamp of ownership
    pub timestamp: i64,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// Whether ownership is verified
    pub verified: bool,
    /// Asset hash
    pub asset_hash: String,
}

/// Ownership history response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipHistory {
    pub asset_id: String,
    pub current_owner: String,
    pub history: Vec<OwnershipTransferEvent>,
}

/// Ownership transfer event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipTransferEvent {
    pub from_address: String,
    pub to_address: String,
    pub timestamp: i64,
    pub tx_hash: String,
    pub block_number: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_ownership_record_timestamp() {
        let record = OwnershipRecord {
            asset_id: "asset-123".to_string(),
            owner_address: "0x1234567890abcdef".to_string(),
            timestamp: 1609459200000, // 2021-01-01 00:00:00 UTC
            tx_hash: "0xabc123".to_string(),
            block_number: 12345,
            asset_hash: "hash123".to_string(),
            ipfs_cid: None,
        };

        let dt = record.timestamp_datetime();
        assert_eq!(dt.year(), 2021);
    }
}
