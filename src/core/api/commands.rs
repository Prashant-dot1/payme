use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::infrastructure::kafka::KafkaProducer;
use crate::core::models::TransactionStatus;
use crate::core::events::TransactionCreatedEvent;

/*request payload types - this is from the user*/
#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    amount: u64,
    currency: String,
    merchant_id: String,
    customer_id: String
}

/*response payload types*/

#[derive(Serialize)]
pub struct CreateTransactionResponse {
    id : Uuid,
    status: TransactionStatus
}

/* Handlers */
pub async fn create_transaction(Json(req_payload): Json<CreateTransactionRequest>) -> Json<CreateTransactionResponse> {

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


    Json(CreateTransactionResponse {
        id: transaction_id,
        status: TransactionStatus::Pending
    })

}