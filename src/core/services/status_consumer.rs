use rdkafka::{
    consumer::{StreamConsumer, Consumer},
    ClientConfig,
    Message
};
use crate::core::events::PaymentStatusUpdatedEvent;

pub struct StatusConsumer {
    consumer: StreamConsumer,
}

impl StatusConsumer {
    pub fn new(kf_broker: &str) -> Self {
        // Initialize Kafka consumer
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "payment-status-consumer")
            .set("bootstrap.servers", kf_broker)
            .set("enable.auto.commit", "true")
            .create()
            .expect("Failed to create consumer");

        Self { consumer }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting payment status consumer service...");

        self.consumer.subscribe(&["payment-status"])
            .expect("Failed to subscribe to payment-status topic");

        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    if let Some(payload) = msg.payload() {
                        match serde_json::from_slice::<PaymentStatusUpdatedEvent>(payload) {
                            Ok(event) => {
                                // 1. Update the query database with new status
                                // 2. Cache the status for quick retrieval
                                println!("Status updated for transaction: {}", event.transaction_id);
                                println!("New status: {:?}", event.status);
                                println!("Stripe payment ID: {}", event.stripe_payment_id);
                            }
                            Err(e) => eprintln!("Failed to deserialize status event: {}", e)
                        }
                    }
                }
                Err(e) => eprintln!("Failed to receive message: {}", e)
            }
        }
    }
} 