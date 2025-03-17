use axum::{
    body::{to_bytes, Body, Bytes}, extract::Request, http::header, Json
};
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

pub async fn create_transaction(request: Request<Body>) -> Json<CreateTransactionResponse> {
    // Extract idempotency key from headers FIRST
    let headers = request.headers().clone();
    let idempotency_key = headers
        .get("x-idempotency-key")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    // Check idempotency
    if let Some(cached_response) = check_idempotency_key(idempotency_key).await {
        return Json(cached_response);
    }

    // NOW we can consume the body
    let body = request.into_body();
    let body_bytes = to_bytes(body, usize::MAX).await.expect("Failed to parse the body in bytes");

    
    let req_payload: CreateTransactionRequest = serde_json::from_slice(&body_bytes)
        .expect("Failed to parse request");

    let transaction_id = Uuid::new_v4();
    let event = TransactionCreatedEvent::new(
        transaction_id,
        req_payload.amount,
        req_payload.currency,
        req_payload.merchant_id,
        req_payload.customer_id,
    );


    let producer = KafkaProducer::new("localhost:9092", "transactions");

    if let Err(e) = producer.publish_event(&event).await {
        eprintln!("Failed to publish event to topic: {}", e);
    }

    let response = CreateTransactionResponse {
        id: transaction_id,
        status: TransactionStatus::Pending,
    };

    cache_response(idempotency_key, &response).await;

    Json(response)
}

async fn check_idempotency_key(key: &str) -> Option<CreateTransactionResponse> {
    // In production, this would check Redis/database
    // Return cached response if key exists
    None
}

async fn cache_response(key: &str, response: &CreateTransactionResponse) {
    // In production, store in Redis/database with TTL
}