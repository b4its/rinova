use serde::Serialize;
use chrono::Utc;
use uuid::Uuid;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

/// Message envelope for ZeroMQ publishing
#[derive(Debug, serde::Serialize)]
struct ZmqMessage<T> {
    correlation_id: String,
    timestamp: String,
    topic: String,
    payload: T,
}

/// ZeroMQ Publisher that works in multi-threaded environments
/// Uses a channel to communicate with a dedicated publisher thread
#[derive(Clone)]
pub struct ZeroMQPublisher {
    sender: Sender<(String, String)>, // (topic, json_payload)
}

impl ZeroMQPublisher {
    pub fn new(url: &str) -> Result<Self, zmq::Error> {
        let (sender, receiver): (Sender<(String, String)>, Receiver<(String, String)>) = channel();
        
        // Spawn a dedicated thread for ZeroMQ publishing
        let url = url.to_string();
        thread::spawn(move || {
            let context = zmq::Context::new();
            let socket = match context.socket(zmq::PUB) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Failed to create ZeroMQ socket: {}", e);
                    return;
                }
            };
            
            if let Err(e) = socket.connect(&url) {
                tracing::error!("Failed to connect ZeroMQ socket to {}: {}", url, e);
                return;
            }
            
            tracing::info!("ZeroMQ publisher connected to {}", url);
            
            // Process messages from the channel
            while let Ok((topic, json)) = receiver.recv() {
                if let Err(e) = socket.send(topic.as_bytes(), zmq::SNDMORE)
                    .and_then(|_| socket.send(json.as_bytes(), 0))
                {
                    tracing::error!("Failed to publish message to {}: {}", topic, e);
                } else {
                    tracing::debug!("Published message to topic: {}", topic);
                }
            }
        });

        Ok(Self { sender })
    }

    pub async fn publish<T: Serialize>(&self, topic: &str, payload: T) {
        let message = ZmqMessage {
            correlation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            topic: topic.to_string(),
            payload,
        };

        match serde_json::to_string(&message) {
            Ok(json) => {
                if let Err(e) = self.sender.send((topic.to_string(), json)) {
                    tracing::error!("Failed to send message to ZeroMQ thread: {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to serialize message: {}", e);
            }
        }
    }
}
