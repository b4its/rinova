//! Event models for ZeroMQ messages

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event types received from other microservices via ZeroMQ
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum Event {
    /// User registered
    UserRegistered {
        user_id: Uuid,
        email: String,
        account_type: String,
    },
    /// User logged in
    UserLoggedIn {
        user_id: Uuid,
        ip_address: String,
    },
    /// Workspace created
    WorkspaceCreated {
        workspace_id: Uuid,
        owner_id: Uuid,
        name: String,
    },
    /// Member invited to workspace
    MemberInvited {
        workspace_id: Uuid,
        user_id: Uuid,
        email: String,
        role: String,
    },
    /// Member joined workspace
    MemberJoined {
        workspace_id: Uuid,
        user_id: Uuid,
    },
    /// Subscription upgraded
    SubscriptionUpgraded {
        user_id: Uuid,
        old_plan: String,
        new_plan: String,
    },
    /// Subscription downgraded
    SubscriptionDowngraded {
        user_id: Uuid,
        old_plan: String,
        new_plan: String,
    },
    /// Project created
    ProjectCreated {
        project_id: Uuid,
        workspace_id: Uuid,
        user_id: Uuid,
    },
    /// Project updated
    ProjectUpdated {
        project_id: Uuid,
        user_id: Uuid,
        changes: Vec<String>,
    },
    /// Publish started
    PublishStarted {
        project_id: Uuid,
        user_id: Uuid,
    },
    /// Publish completed
    PublishCompleted {
        project_id: Uuid,
        url: String,
        audit_hash: String,
    },
    /// Publish failed
    PublishFailed {
        project_id: Uuid,
        error: String,
    },
    /// Blockchain transaction confirmed
    BlockchainConfirmed {
        tx_hash: String,
        operation: String,
    },
    /// Ownership recorded
    OwnershipRecorded {
        asset_id: String,
        owner_address: String,
        tx_hash: String,
    },
    /// Audit trail recorded
    AuditTrailRecorded {
        project_id: Uuid,
        hash: String,
        tx_hash: String,
    },
}

/// ZeroMQ message envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZmqMessage {
    /// Correlation ID for tracing
    pub correlation_id: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Topic
    pub topic: String,
    /// Event payload
    pub event: Event,
}

impl ZmqMessage {
    /// Create a new ZMQ message
    pub fn new(topic: String, event: Event) -> Self {
        ZmqMessage {
            correlation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            topic,
            event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zmq_message_creation() {
        let event = Event::ProjectCreated {
            project_id: Uuid::new_v4(),
            workspace_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
        };

        let message = ZmqMessage::new("projects".to_string(), event);

        assert!(!message.correlation_id.is_empty());
        assert_eq!(message.topic, "projects");
    }
}
