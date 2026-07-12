//! Notification orchestration service

use anyhow::Result;
use uuid::Uuid;

use crate::models::{Event, Notification, NotificationType};
use super::{NotificationStore, EmailService, WebSocketManager};

/// Notification service for orchestrating notifications
pub struct NotificationService {
    store: NotificationStore,
    email_service: EmailService,
    ws_manager: WebSocketManager,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(
        store: NotificationStore,
        email_service: EmailService,
        ws_manager: WebSocketManager,
    ) -> Self {
        NotificationService {
            store,
            email_service,
            ws_manager,
        }
    }

    /// Process an event from ZeroMQ
    pub async fn process_event(&self, event: Event) -> Result<()> {
        tracing::info!("Processing event: {:?}", event);

        match event {
            Event::MemberInvited {
                workspace_id,
                user_id,
                email,
                role,
            } => {
                self.handle_member_invited(workspace_id, user_id, &email, &role)
                    .await?;
            }
            Event::MemberJoined {
                workspace_id,
                user_id,
            } => {
                self.handle_member_joined(workspace_id, user_id).await?;
            }
            Event::PublishCompleted {
                project_id,
                url,
                audit_hash,
            } => {
                self.handle_publish_completed(project_id, &url, &audit_hash)
                    .await?;
            }
            Event::PublishFailed { project_id, error } => {
                self.handle_publish_failed(project_id, &error).await?;
            }
            Event::BlockchainConfirmed {
                tx_hash,
                operation,
            } => {
                self.handle_blockchain_confirmed(&tx_hash, &operation).await?;
            }
            Event::SubscriptionUpgraded {
                user_id,
                old_plan,
                new_plan,
            } => {
                self.handle_subscription_upgraded(user_id, &old_plan, &new_plan)
                    .await?;
            }
            Event::SubscriptionDowngraded {
                user_id,
                old_plan,
                new_plan,
            } => {
                self.handle_subscription_downgraded(user_id, &old_plan, &new_plan)
                    .await?;
            }
            _ => {
                tracing::debug!("Event type not handled for notification");
            }
        }

        Ok(())
    }

    /// Handle member invited event
    async fn handle_member_invited(
        &self,
        workspace_id: Uuid,
        user_id: Uuid,
        email: &str,
        role: &str,
    ) -> Result<()> {
        let notification = Notification::new(
            user_id,
            NotificationType::WorkspaceInvitation,
            "You've been invited to join a workspace".to_string(),
            format!("You have been invited to join a workspace as {}", role),
            Some(serde_json::json!({
                "workspace_id": workspace_id,
                "role": role
            })),
        );

        self.send_notification(notification).await?;

        // Send email notification
        self.email_service
            .send_workspace_invitation(email, workspace_id, role)
            .await?;

        Ok(())
    }

    /// Handle member joined event
    async fn handle_member_joined(&self, workspace_id: Uuid, user_id: Uuid) -> Result<()> {
        // Notify workspace owner
        let notification = Notification::new(
            user_id,
            NotificationType::MemberJoined,
            "New member joined".to_string(),
            "A new member has joined your workspace".to_string(),
            Some(serde_json::json!({
                "workspace_id": workspace_id
            })),
        );

        self.send_notification(notification).await?;

        Ok(())
    }

    /// Handle publish completed event
    async fn handle_publish_completed(
        &self,
        project_id: Uuid,
        url: &str,
        audit_hash: &str,
    ) -> Result<()> {
        // Note: Would need to get project owner from Project Service
        let notification = Notification::new(
            Uuid::nil(), // Placeholder
            NotificationType::PublishCompleted,
            "Your website is live!".to_string(),
            format!("Your website has been published successfully at {}", url),
            Some(serde_json::json!({
                "project_id": project_id,
                "url": url,
                "audit_hash": audit_hash
            })),
        );

        self.send_notification(notification).await?;

        Ok(())
    }

    /// Handle publish failed event
    async fn handle_publish_failed(&self, project_id: Uuid, error: &str) -> Result<()> {
        let notification = Notification::new(
            Uuid::nil(), // Placeholder
            NotificationType::PublishFailed,
            "Publish failed".to_string(),
            format!("Failed to publish your website: {}", error),
            Some(serde_json::json!({
                "project_id": project_id,
                "error": error
            })),
        );

        self.send_notification(notification).await?;

        Ok(())
    }

    /// Handle blockchain confirmed event
    async fn handle_blockchain_confirmed(&self, tx_hash: &str, operation: &str) -> Result<()> {
        tracing::info!("Blockchain transaction confirmed: {} ({})", tx_hash, operation);
        // Would send notification to relevant user
        Ok(())
    }

    /// Handle subscription upgraded
    async fn handle_subscription_upgraded(
        &self,
        user_id: Uuid,
        old_plan: &str,
        new_plan: &str,
    ) -> Result<()> {
        let notification = Notification::new(
            user_id,
            NotificationType::SubscriptionUpgraded,
            "Plan upgraded!".to_string(),
            format!(
                "Your subscription has been upgraded from {} to {}",
                old_plan, new_plan
            ),
            Some(serde_json::json!({
                "old_plan": old_plan,
                "new_plan": new_plan
            })),
        );

        self.send_notification(notification).await?;

        Ok(())
    }

    /// Handle subscription downgraded
    async fn handle_subscription_downgraded(
        &self,
        user_id: Uuid,
        old_plan: &str,
        new_plan: &str,
    ) -> Result<()> {
        let notification = Notification::new(
            user_id,
            NotificationType::SubscriptionExpired,
            "Plan changed".to_string(),
            format!(
                "Your subscription has been changed from {} to {}",
                old_plan, new_plan
            ),
            Some(serde_json::json!({
                "old_plan": old_plan,
                "new_plan": new_plan
            })),
        );

        self.send_notification(notification).await?;

        Ok(())
    }

    /// Send notification (store + real-time push)
    async fn send_notification(&self, notification: Notification) -> Result<()> {
        let user_id = notification.user_id;

        // Store notification
        self.store.store(&notification).await?;

        // Push via WebSocket if user is online
        self.ws_manager.send_to_user(user_id, &notification).await;

        tracing::info!("Sent notification {} to user {}", notification.id, user_id);

        Ok(())
    }

    /// List notifications for a user
    pub async fn list_notifications(
        &self,
        user_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<crate::models::NotificationListResponse> {
        self.store.list(user_id, page, page_size).await
    }

    /// Mark notifications as read
    pub async fn mark_read(&self, user_id: Uuid, notification_ids: &[Uuid]) -> Result<u64> {
        self.store.mark_read(user_id, notification_ids).await
    }
}
