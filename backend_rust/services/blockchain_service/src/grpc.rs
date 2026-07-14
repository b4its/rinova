//! gRPC server exposing blockchain operations to other internal services.

use tonic::{Request, Response, Status};

use proto::blockchain::blockchain_service_server::BlockchainService as GrpcBlockchainService;
use proto::blockchain::{
    GetSubscriptionHistoryRequest, RecordSubscriptionRequest as GrpcRecordSubscriptionRequest,
    SubscriptionHistory, SubscriptionRecord as GrpcSubscriptionRecord,
};

use crate::models::RecordSubscriptionRequest;
use crate::services::BlockchainClient;

/// gRPC adapter around the existing BlockchainClient.
pub struct BlockchainGrpc {
    client: BlockchainClient,
}

impl BlockchainGrpc {
    pub fn new(client: BlockchainClient) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl GrpcBlockchainService for BlockchainGrpc {
    async fn record_subscription(
        &self,
        request: Request<GrpcRecordSubscriptionRequest>,
    ) -> Result<Response<GrpcSubscriptionRecord>, Status> {
        let r = request.into_inner();

        if r.user_id.is_empty() || r.subscription_id.is_empty() {
            return Err(Status::invalid_argument(
                "user_id and subscription_id are required",
            ));
        }

        let record = self
            .client
            .record_subscription(RecordSubscriptionRequest {
                user_id: r.user_id,
                subscription_id: r.subscription_id,
                plan_type: r.plan_type,
                action: r.action,
                amount_cents: r.amount_cents,
                currency: if r.currency.is_empty() {
                    "usd".to_string()
                } else {
                    r.currency
                },
                payment_reference: r.payment_reference,
            })
            .await
            .map_err(|e| Status::internal(format!("record failed: {e}")))?;

        Ok(Response::new(GrpcSubscriptionRecord {
            user_id: record.user_id,
            subscription_id: record.subscription_id,
            plan_type: record.plan_type,
            action: record.action,
            amount_cents: record.amount_cents,
            currency: record.currency,
            payment_reference: record.payment_reference,
            content_hash: record.content_hash,
            tx_hash: record.tx_hash,
            block_number: record.block_number,
            timestamp: record.timestamp,
        }))
    }

    async fn get_subscription_history(
        &self,
        request: Request<GetSubscriptionHistoryRequest>,
    ) -> Result<Response<SubscriptionHistory>, Status> {
        let r = request.into_inner();
        let records = self
            .client
            .get_subscription_history(&r.user_id, r.limit)
            .await
            .map_err(|e| Status::internal(format!("query failed: {e}")))?;

        let mapped: Vec<GrpcSubscriptionRecord> = records
            .into_iter()
            .map(|record| GrpcSubscriptionRecord {
                user_id: record.user_id,
                subscription_id: record.subscription_id,
                plan_type: record.plan_type,
                action: record.action,
                amount_cents: record.amount_cents,
                currency: record.currency,
                payment_reference: record.payment_reference,
                content_hash: record.content_hash,
                tx_hash: record.tx_hash,
                block_number: record.block_number,
                timestamp: record.timestamp,
            })
            .collect();

        Ok(Response::new(SubscriptionHistory {
            user_id: r.user_id,
            total: mapped.len() as u64,
            records: mapped,
        }))
    }
}
