use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use serde::Serialize;
use std::time::Duration;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation failed");

        Self {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn publish_event<T: Serialize>(&self, event: &T) -> Result<(), String> {
        let payload = serde_json::to_string(&event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;

        self.producer
            .send(
                FutureRecord::to(&self.topic)
                    .payload(&payload)
                    .key(""), // You might want to set a key based on transaction_id
                Duration::from_secs(5),
            )
            .await
            .map_err(|(e, _)| format!("Failed to send message: {}", e))?;

        Ok(())
    }
} 