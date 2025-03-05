use serde::Deserialize;

/*request payload types - this is from the user*/
#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    amount: u64,
    currency: String,
    payment_method_id: String,
    customer_id: String
}

/*response payload types*/