use std::{collections::HashMap, str::FromStr};
use stripe::{Client, CreatePaymentIntent, PaymentIntent, StripeError};

use crate::core::events::TransactionCreatedEvent;

pub struct StripeService {
    client: Client
}

impl StripeService {
    pub fn new(stripe_secret_key: &str) -> Self {
        let client = Client::new(stripe_secret_key);

        Self{
            client
        }
    }

    pub async fn process_payment(&self, event: TransactionCreatedEvent) -> 
        Result<PaymentIntent, StripeError> {

            let _metadata = [
                ("transaction_id".to_string(), event.transaction_id.to_string()),
                ("merchant_id".to_string(), event.merchant_id.clone()),
                ("customer_id".to_string(), event.customer_id.clone()),
            ].iter()
            .cloned()
            .collect::<HashMap<String, String>>();


            let params = CreatePaymentIntent::new(event.amount as i64, stripe::Currency::from_str(&event.currency).unwrap());

            PaymentIntent::create(&self.client, params).await
    }
}