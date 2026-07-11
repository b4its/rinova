use zmq::{Context, Socket};
use serde::Serialize;
use chrono::Utc;
use uuid::Uuid;

pub struct ZeroMQPublisher {
    socket: Socket,
}

impl ZeroMQPublisher {
    pub fn new(url: &str) -> Result<Self, zmq::Error> {
        let context = Context::new();
        let socket = context.socket(zmq::PUB)?;
        socket.connect(url)?;
        
        Ok(Self { socket })
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
                if let Err(e) = self.socket.send(topic.as_bytes(), zmq::SNDMORE)
                    .and_then(|_| self.socket.send(json.as_bytes(), 0))
                {
                    tracing::error!("Failed to publish message to {}: {}", topic, e);
                } else {
                    tracing::debug!("Published message to topic: {}", topic);
                }
            }
            Err(e) => {
                tracing::error!("Failed to serialize message: {}", e);
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct ZmqMessage<T> {
    correlation_id: String,
    timestamp: String,
    topic: String,
    payload: T,
}
