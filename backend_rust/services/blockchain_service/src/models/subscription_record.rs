//! Subscription transaction records for on-chain audit.
//!
//! Each subscription change (upgrade/downgrade/cancel/renew) is hashed and
//! recorded on the Besu chain so it can be independently verified later.

use serde::{Deserialize, Serialize};

/// Request to record a subscription transaction on-chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSubscriptionRequest {
    /// User the subscription belongs to.
    pub user_id: String,
    /// Subscription ID (from PostgreSQL).
    pub subscription_id: String,
    /// Plan being moved to (freemium|enterprise|exclusive).
    pub plan_type: String,
    /// Action performed (upgrade|downgrade|cancel|renew|create).
    pub action: String,
    /// Amount charged, in the smallest currency unit (cents). 0 for free.
    #[serde(default)]
    pub amount_cents: i64,
    /// Currency code (e.g. "usd").
    #[serde(default = "default_currency")]
    pub currency: String,
    /// Optional external payment reference (e.g. Stripe payment intent).
    pub payment_reference: Option<String>,
}

fn default_currency() -> String {
    "usd".to_string()
}

/// A subscription transaction record as stored on-chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRecord {
    pub user_id: String,
    pub subscription_id: String,
    pub plan_type: String,
    pub action: String,
    pub amount_cents: i64,
    pub currency: String,
    pub payment_reference: Option<String>,
    /// SHA-256 hash of the canonical transaction payload.
    pub content_hash: String,
    /// On-chain transaction hash.
    pub tx_hash: String,
    /// Block number the tx was included in.
    pub block_number: u64,
    /// Unix epoch milliseconds.
    pub timestamp: i64,
}

/// Response listing a user's subscription records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionHistory {
    pub user_id: String,
    pub records: Vec<SubscriptionRecord>,
    pub total: usize,
}
