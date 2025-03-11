use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::TransactionStatus;

#[derive(Serialize, Deserialize,Clone)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentStatusUpdatedEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub timestamp : DateTime<Utc>,
    pub transaction_id: Uuid,
    pub status: TransactionStatus,
    pub stripe_payment_id: String
}

impl PaymentStatusUpdatedEvent {
    pub fn new(transaction_id: Uuid, status: TransactionStatus , stripe_payment_id: String) -> Self {
        Self { event_id: Uuid::new_v4(), event_type: "STATUS_UPDATED".to_string(), timestamp: Utc::now(), transaction_id, status, stripe_payment_id }
    }
}

pub struct PaymentStatusRequestEvent {
    pub event_id: Uuid,
    pub evnet_type: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_id: Uuid,
    pub reply_topic: String
}

impl PaymentStatusRequestEvent{
    pub fn new(transaction_id: Uuid) -> Self {
        Self { event_id: Uuid::new_v4(), evnet_type: "STATUS_REQUEST".to_string(), timestamp: Utc::now(), transaction_id , reply_topic: "payment-status-response".to_string() }
    }
}