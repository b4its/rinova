//! Notification models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Notification types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    /// Workspace invitation
    WorkspaceInvitation,
    /// Member joined workspace
    MemberJoined,
    /// Project created
    ProjectCreated,
    /// Publish completed
    PublishCompleted,
    /// Publish failed
    PublishFailed,
    /// Blockchain transaction confirmed
    BlockchainConfirmed,
    /// Subscription upgraded
    SubscriptionUpgraded,
    /// Subscription expired
    SubscriptionExpired,
    /// System announcement
    SystemAnnouncement,
}

/// Notification model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Unique notification ID
    pub id: Uuid,
    /// User ID who should receive this notification
    pub user_id: Uuid,
    /// Notification type
    pub notification_type: NotificationType,
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Additional data (JSON)
    pub data: Option<serde_json::Value>,
    /// Whether the notification has been read
    pub read: bool,
    /// When the notification was created
    pub created_at: DateTime<Utc>,
}

impl Notification {
    /// Create a new notification
    pub fn new(
        user_id: Uuid,
        notification_type: NotificationType,
        title: String,
        message: String,
        data: Option<serde_json::Value>,
    ) -> Self {
        Notification {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            title,
            message,
            data,
            read: false,
            created_at: Utc::now(),
        }
    }
}

/// Notification list response with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<Notification>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub unread_count: u64,
}

/// Request to mark notifications as read
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkReadRequest {
    pub notification_ids: Vec<Uuid>,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSocketMessage {
    /// Client subscribes to notifications
    Subscribe { user_id: Uuid },
    /// New notification
    Notification { notification: Notification },
    /// Notification read status update
    ReadUpdate { notification_ids: Vec<Uuid> },
    /// Heartbeat
    Ping,
    /// Heartbeat response
    Pong,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub push_enabled: bool,
    pub workspace_invitations: bool,
    pub publish_notifications: bool,
    pub blockchain_notifications: bool,
    pub subscription_notifications: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        NotificationPreferences {
            user_id: Uuid::nil(),
            email_enabled: true,
            push_enabled: true,
            workspace_invitations: true,
            publish_notifications: true,
            blockchain_notifications: true,
            subscription_notifications: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_new() {
        let user_id = Uuid::new_v4();
        let notification = Notification::new(
            user_id,
            NotificationType::WorkspaceInvitation,
            "You've been invited".to_string(),
            "You have been invited to join workspace X".to_string(),
            Some(serde_json::json!({ "workspace_id": "123" })),
        );

        assert_eq!(notification.user_id, user_id);
        assert!(!notification.read);
        assert_eq!(notification.notification_type, NotificationType::WorkspaceInvitation);
    }
}
