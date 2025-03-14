use std::time::Duration;

use axum::{
    extract::Path,
    Json,
};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use uuid::Uuid;

use crate::core::events::PaymentStatusRequestEvent;


pub struct Query {
    producer: FutureProducer
}

impl Query {
    pub fn new(kf_broker: &str) -> Self {
        let producer = ClientConfig::new()
                    .set("bootstrap.servers", kf_broker)
                    .set("message.timeout.ms", "5000")
                    .create()
                    .expect("Failed to create a producer");
        
        Self { producer }
    }

    pub async fn get_payment_status(
        &self,
        idempotency_key: String,
        Path(transaction_id): Path<Uuid>
    ) -> Json<()> {
        // Check idempotency
        if let Some(cached_response) = self.check_idempotency(&idempotency_key).await {
            return cached_response;
        }

        let req_event = PaymentStatusRequestEvent::new(transaction_id);

        let payload = serde_json::to_string(&req_event).unwrap();

        match self.producer
        .send(
            FutureRecord::to("payment-status-requests").payload(&payload).key(&transaction_id.to_string()), Duration::from_secs(5)).await {
                
                Ok(_) => {
                    todo!("database integration needed here")
                },
                Err(e) => {
                    eprintln!("Failed to publish the status req event to event bus: {}", e.0);

                    todo!("need to return a valid json response here");
                }
        }
    }

    async fn check_idempotency(&self, key: &str) -> Option<Json<()>> {
        // Check cache/database for previous response
        None
    }
}

