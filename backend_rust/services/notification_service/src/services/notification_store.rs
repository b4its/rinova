//! Notification storage service

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use uuid::Uuid;

use crate::models::{Notification, NotificationListResponse, NotificationType};

/// MongoDB document for notification storage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NotificationDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    notification_id: String,
    user_id: String,
    notification_type: String,
    title: String,
    message: String,
    data: Option<serde_json::Value>,
    read: bool,
    created_at: DateTime<Utc>,
}

/// Notification store for MongoDB
pub struct NotificationStore {
    collection: Collection<NotificationDocument>,
}

impl NotificationStore {
    /// Create a new notification store
    pub async fn new(mongodb_uri: &str, database_name: &str) -> Result<Self> {
        let client = Client::new(mongodb_uri)
            .await
            .context("Failed to connect to MongoDB")?;

        let database = client.database(database_name);
        let collection = database.collection("notifications");

        // Create indexes
        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "user_id": 1, "created_at": -1 })
                    .build(),
            )
            .await?;

        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "user_id": 1, "read": 1 })
                    .build(),
            )
            .await?;

        Ok(NotificationStore { collection })
    }

    /// Store a new notification
    pub async fn store(&self, notification: &Notification) -> Result<()> {
        let doc = NotificationDocument {
            id: None,
            notification_id: notification.id.to_string(),
            user_id: notification.user_id.to_string(),
            notification_type: serde_json::to_string(&notification.notification_type)?,
            title: notification.title.clone(),
            message: notification.message.clone(),
            data: notification.data.clone(),
            read: notification.read,
            created_at: notification.created_at,
        };

        self.collection
            .insert_one(&doc)
            .await
            .context("Failed to store notification")?;

        Ok(())
    }

    /// List notifications for a user with pagination
    pub async fn list(
        &self,
        user_id: Uuid,
        page: u32,
        page_size: u32,
    ) -> Result<NotificationListResponse> {
        let skip = ((page - 1) * page_size) as u64;

        // Count total
        let total = self.collection
            .count_documents(doc! { "user_id": user_id.to_string() })
            .await?;

        // Count unread
        let unread_count = self.collection
            .count_documents(doc! {
                "user_id": user_id.to_string(),
                "read": false
            })
            .await?;

        // Fetch notifications
        let mut cursor = self.collection
            .find(doc! { "user_id": user_id.to_string() })
            .sort(doc! { "created_at": -1 })
            .skip(skip)
            .limit(page_size as i64)
            .await?;

        let mut notifications = Vec::new();
        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            notifications.push(Notification {
                id: Uuid::parse_str(&doc.notification_id)?,
                user_id: Uuid::parse_str(&doc.user_id)?,
                notification_type: serde_json::from_str(&doc.notification_type)?,
                title: doc.title,
                message: doc.message,
                data: doc.data,
                read: doc.read,
                created_at: doc.created_at,
            });
        }

        let total_pages = ((total as f64) / (page_size as f64)).ceil() as u32;

        Ok(NotificationListResponse {
            notifications,
            total,
            page,
            page_size,
            total_pages,
            unread_count,
        })
    }

    /// Mark notifications as read
    pub async fn mark_read(&self, user_id: Uuid, notification_ids: &[Uuid]) -> Result<u64> {
        let ids: Vec<String> = notification_ids.iter().map(|id| id.to_string()).collect();

        let result = self.collection
            .update_many(
                doc! {
                    "user_id": user_id.to_string(),
                    "notification_id": { "$in": ids }
                },
                doc! { "$set": { "read": true } },
            )
            .await?;

        Ok(result.modified_count)
    }

    /// Delete old notifications (retention policy: 30 days)
    pub async fn cleanup_old_notifications(&self) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(30);

        let result = self.collection
            .delete_many(doc! {
                "created_at": { "$lt": cutoff }
            })
            .await?;

        Ok(result.deleted_count)
    }

    /// Get unread count for a user
    pub async fn get_unread_count(&self, user_id: Uuid) -> Result<u64> {
        let count = self.collection
            .count_documents(doc! {
                "user_id": user_id.to_string(),
                "read": false
            })
            .await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests require a test MongoDB instance
}
