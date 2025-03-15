use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum_extra::{
    headers::{Header, HeaderName},
    TypedHeader,
};

use crate::core::{infrastructure::kafka::KafkaProducer, models::IdempotencyKey};
use crate::core::models::TransactionStatus;
use crate::core::events::TransactionCreatedEvent;

/*request payload types - this is from the user*/
#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    amount: u64,
    currency: String,
    merchant_id: String,
    customer_id: String,
    idempotency_key: String,  // Client-provided idempotency key
}

/*response payload types*/

#[derive(Serialize)]
pub struct CreateTransactionResponse {
    id : Uuid,
    status: TransactionStatus
}

static IDEMPOTENCY_KEY: HeaderName = HeaderName::from_static("x-idempotency-key");

/* Handlers */
pub async fn create_transaction(
    TypedHeader(idempotency_key): TypedHeader<IdempotencyKey>,
    Json(req_payload): Json<CreateTransactionRequest>
) -> Json<CreateTransactionResponse> {
    // Check if we've seen this idempotency key before
    if let Some(cached_response) = check_idempotency_key(&idempotency_key.0).await {
        return Json(cached_response);
    }

    let transaction_id = Uuid::new_v4();
    let event = TransactionCreatedEvent::new(
        transaction_id,
        req_payload.amount,
        req_payload.currency,
        req_payload.merchant_id,
        req_payload.customer_id
    );


    let producer = KafkaProducer::new("localhost:9092", "transactions");

    if let Err(e) = producer.publish_event(&event).await {
        eprintln!("Failed to publish event to topic: {}", e);
    }

    // Store the response with the idempotency key
    cache_response(&idempotency_key.0, &CreateTransactionResponse {
        id: transaction_id,
        status: TransactionStatus::Pending
    }).await;

    Json(CreateTransactionResponse {
        id: transaction_id,
        status: TransactionStatus::Pending
    })
}

async fn check_idempotency_key(key: &str) -> Option<CreateTransactionResponse> {
    // In production, this would check Redis/database
    // Return cached response if key exists
    None
}

async fn cache_response(key: &str, response: &CreateTransactionResponse) {
    // In production, store in Redis/database with TTL
}