//! gRPC client for recording subscription transactions on the blockchain.
//!
//! Calls are best-effort: if the blockchain service is unavailable, we log a
//! warning and continue, so subscription changes are never blocked by it.

use proto::blockchain::blockchain_service_client::BlockchainServiceClient;
use proto::blockchain::RecordSubscriptionRequest;

/// Record a subscription transaction on-chain. Returns the tx hash on success.
pub async fn record_subscription_tx(
    user_id: &str,
    subscription_id: &str,
    plan_type: &str,
    action: &str,
    amount_cents: i64,
) -> Option<String> {
    let endpoint = std::env::var("BLOCKCHAIN_GRPC_URL")
        .unwrap_or_else(|_| "http://blockchain-service:9005".to_string());

    let mut client = match BlockchainServiceClient::connect(endpoint).await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Blockchain gRPC unavailable, skipping on-chain record: {}", e);
            return None;
        }
    };

    let request = tonic::Request::new(RecordSubscriptionRequest {
        user_id: user_id.to_string(),
        subscription_id: subscription_id.to_string(),
        plan_type: plan_type.to_string(),
        action: action.to_string(),
        amount_cents,
        currency: "usd".to_string(),
        payment_reference: None,
    });

    match client.record_subscription(request).await {
        Ok(resp) => {
            let record = resp.into_inner();
            tracing::info!(
                "Subscription tx recorded on-chain: user={}, action={}, tx={}",
                user_id, action, record.tx_hash
            );
            Some(record.tx_hash)
        }
        Err(e) => {
            tracing::warn!("Failed to record subscription tx on-chain: {}", e);
            None
        }
    }
}
