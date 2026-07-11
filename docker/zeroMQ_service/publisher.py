#!/usr/bin/env python3
"""
Rinova Website Builder - ZeroMQ Publisher Service
Message bus for async communication between microservices
"""

import zmq
import json
import time
import uuid
from datetime import datetime
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class ZeroMQPublisher:
    def __init__(self):
        self.context = zmq.Context()
        
        # Publisher socket for sending messages
        self.publisher = self.context.socket(zmq.PUB)
        self.publisher.bind("tcp://*:5555")
        logger.info("ZeroMQ Publisher bound to tcp://*:5555")
        
        # Subscriber socket for receiving (for feedback)
        self.subscriber = self.context.socket(zmq.SUB)
        self.subscriber.bind("tcp://*:5556")
        self.subscriber.setsockopt_string(zmq.SUBSCRIBE, "")
        logger.info("ZeroMQ Subscriber bound to tcp://*:5556")
        
    def publish_message(self, topic: str, payload: dict):
        """
        Publish a message to a specific topic
        
        Args:
            topic: Message topic (e.g., 'user.registered', 'project.created')
            payload: Message payload dictionary
        """
        message = {
            "correlation_id": str(uuid.uuid4()),
            "timestamp": datetime.utcnow().isoformat(),
            "topic": topic,
            "payload": payload
        }
        
        try:
            # Send topic as multipart message
            self.publisher.send_string(topic, zmq.SNDMORE)
            self.publisher.send_string(json.dumps(message))
            logger.info(f"Published message to topic '{topic}': {message['correlation_id']}")
            return True
        except Exception as e:
            logger.error(f"Failed to publish message: {e}")
            return False
    
    def run(self):
        """Run the ZeroMQ publisher service"""
        logger.info("ZeroMQ Publisher Service started")
        
        try:
            while True:
                time.sleep(1)
        except KeyboardInterrupt:
            logger.info("Shutting down ZeroMQ Publisher Service...")
        finally:
            self.publisher.close()
            self.subscriber.close()
            self.context.term()


if __name__ == "__main__":
    publisher = ZeroMQPublisher()
    publisher.run()
