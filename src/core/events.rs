use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TransactionCreatedEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_id: Uuid,
    pub amount: u64,
    pub currency: String,
    pub merchant_id: String,
    pub customer_id: String,
}

impl TransactionCreatedEvent {
    pub fn new(
        transaction_id: Uuid,
        amount: u64,
        currency: String,
        merchant_id: String,
        customer_id: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type: "TRANSACTION.CREATED".to_string(),
            timestamp: Utc::now(),
            transaction_id,
            amount,
            currency,
            merchant_id,
            customer_id,
        }
    }
} 