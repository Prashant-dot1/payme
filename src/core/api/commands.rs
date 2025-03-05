use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::models::TransactionStatus;

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

    todo!("integration with the command service")
}