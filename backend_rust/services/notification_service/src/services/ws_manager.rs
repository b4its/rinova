//! WebSocket connection manager

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::models::{Notification, WebSocketMessage};

/// WebSocket session handle
pub struct WsSession {
    pub user_id: Uuid,
    pub tx: mpsc::Sender<WebSocketMessage>,
}

/// WebSocket connection manager
#[derive(Clone)]
pub struct WebSocketManager {
    /// Active connections: user_id -> list of sessions
    sessions: Arc<RwLock<HashMap<Uuid, Vec<mpsc::Sender<WebSocketMessage>>>>>,
    /// Offline notification buffer: user_id -> notifications
    offline_buffer: Arc<RwLock<HashMap<Uuid, Vec<Notification>>>>,
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    pub fn new() -> Self {
        WebSocketManager {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            offline_buffer: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new WebSocket connection
    pub async fn register(&self, user_id: Uuid, tx: mpsc::Sender<WebSocketMessage>) {
        let mut sessions = self.sessions.write().await;
        sessions.entry(user_id).or_default().push(tx.clone());

        tracing::info!("WebSocket connection registered for user: {}", user_id);

        // Deliver offline notifications
        let mut buffer = self.offline_buffer.write().await;
        if let Some(notifications) = buffer.remove(&user_id) {
            tracing::info!(
                "Delivering {} offline notifications to user: {}",
                notifications.len(),
                user_id
            );
            for notification in notifications {
                let msg = WebSocketMessage::Notification { notification };
                let _ = tx.send(msg).await;
            }
        }
    }

    /// Unregister a WebSocket connection
    pub async fn unregister(&self, user_id: Uuid, tx: &mpsc::Sender<WebSocketMessage>) {
        let mut sessions = self.sessions.write().await;
        if let Some(user_sessions) = sessions.get_mut(&user_id) {
            user_sessions.retain(|s| !s.same_channel(tx));
            if user_sessions.is_empty() {
                sessions.remove(&user_id);
            }
        }

        tracing::info!("WebSocket connection unregistered for user: {}", user_id);
    }

    /// Send a notification to a specific user
    pub async fn send_to_user(&self, user_id: Uuid, notification: &Notification) {
        let sessions = self.sessions.read().await;

        if let Some(user_sessions) = sessions.get(&user_id) {
            let msg = WebSocketMessage::Notification {
                notification: notification.clone(),
            };

            let mut sent = false;
            for tx in user_sessions {
                if tx.send(msg.clone()).await.is_ok() {
                    sent = true;
                }
            }

            if !sent {
                tracing::warn!("Failed to send notification to user: {}", user_id);
                // Buffer for offline delivery
                drop(sessions);
                self.buffer_notification(user_id, notification.clone()).await;
            }
        } else {
            tracing::debug!("User {} is offline, buffering notification", user_id);
            drop(sessions);
            self.buffer_notification(user_id, notification.clone()).await;
        }
    }

    /// Buffer a notification for offline user
    async fn buffer_notification(&self, user_id: Uuid, notification: Notification) {
        let mut buffer = self.offline_buffer.write().await;
        let user_buffer = buffer.entry(user_id).or_default();

        // Keep max 1000 notifications per user
        if user_buffer.len() >= 1000 {
            user_buffer.remove(0);
        }

        user_buffer.push(notification);
    }

    /// Check if a user is online
    pub async fn is_user_online(&self, user_id: Uuid) -> bool {
        let sessions = self.sessions.read().await;
        sessions.contains_key(&user_id)
    }

    /// Get online user count
    pub async fn online_user_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }

    /// Broadcast a message to all online users
    pub async fn broadcast(&self, msg: WebSocketMessage) {
        let sessions = self.sessions.read().await;

        for (user_id, user_sessions) in sessions.iter() {
            for tx in user_sessions {
                if tx.send(msg.clone()).await.is_err() {
                    tracing::warn!("Failed to broadcast to user: {}", user_id);
                }
            }
        }
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_manager() {
        let manager = WebSocketManager::new();

        assert!(!manager.is_user_online(Uuid::new_v4()).await);
        assert_eq!(manager.online_user_count().await, 0);
    }
}
