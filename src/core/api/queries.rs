use axum::{
    extract::Path,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::core::models::TransactionStatus;


/* response type for the query service */
#[derive(Serialize)]
pub struct PaymentStatusResponse {
    transaction_id: Uuid,
    status: TransactionStatus,
    stripe_payment_id: Option<String>
}

pub async fn get_payment_status(Path(transaction_id): Path<Uuid>) -> Json<PaymentStatusResponse> {

    // for now returing the mock data
    Json(PaymentStatusResponse { 
        transaction_id, 
        status: TransactionStatus::Completed , 
        stripe_payment_id: None })
}
