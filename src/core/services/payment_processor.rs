use rdkafka::{consumer::{Consumer, StreamConsumer}, ClientConfig, Message};

use crate::core::{events::TransactionCreatedEvent, infrastructure::stripe::StripeService};

pub struct PaymentProcessor {
    stripe_service: StripeService,
    consumer: StreamConsumer
}

impl PaymentProcessor {
    pub fn new(kf_broker: &str, stripe_secret_key: &str) -> Self {
        
        let stripe_service = StripeService::new(stripe_secret_key);

        // create the consumer group
        let consumer = ClientConfig::new()
                .set("group.id", "stripe-payment-processor")
                .set("bootstrap.servers", kf_broker)
                .set("enable.auto.commit", "true")
                .create()
                .expect("Consumer creation failed");


        Self {
            stripe_service,
            consumer
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {

        println!("Stripe service starting....");

        self.consumer.subscribe(&["transactions"])
        .expect("Failed to subscribe to the topic");

        
        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    if let Some(paylod) = msg.payload() {
                        match serde_json::from_slice::<TransactionCreatedEvent>(paylod) {
                            Ok(payment_intent) => {
                                println!("Payment processed successfullty");
                            },
                            Err(e) => {
                                eprintln!("Failed to process the payment: {}", e)
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to recv message: {}", e)
                }
            }
        }

    }
}