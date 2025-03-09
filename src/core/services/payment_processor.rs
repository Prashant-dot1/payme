use std::time::Duration;

use rdkafka::{consumer::{Consumer, StreamConsumer}, producer::{FutureProducer, FutureRecord}, ClientConfig, Message};

use crate::core::{events::{PaymentStatusUpdatedEvent, TransactionCreatedEvent}, infrastructure::stripe::StripeService, models::TransactionStatus};

pub struct PaymentProcessor {
    stripe_service: StripeService,
    consumer: StreamConsumer,
    producer: FutureProducer
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

        let producer = ClientConfig::new()
                    .set("bootstrap.servers", kf_broker)
                    .set("message.timeout.ms", "5000")
                    .create()
                    .expect("Failed to created the producer");


        Self {
            stripe_service,
            consumer,
            producer
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
                            Ok(event) => {
                                println!("Processig the transaction id: {}", event.transaction_id);

                                match self.stripe_service.process_payment(event.clone()).await {
                                    Ok(payment_intent) => {
                                        let status_event = PaymentStatusUpdatedEvent::new(event.transaction_id, TransactionStatus::Completed, payment_intent.id.to_string());

                                        self.publish_status_update(status_event).await;
                                        println!("Paymnet processed successfully..");
                                    },
                                    Err(e) => {
                                        let status_event = PaymentStatusUpdatedEvent::new(event.transaction_id, TransactionStatus::Failed {
                                            reason: e.to_string()
                                        }, String::new());

                                        self.publish_status_update(status_event).await;
                                        println!("Failed to process the payment..");
                                    }
                                }
                            },
                            Err(e) => eprintln!("Failed to deserialise event {}", e)
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to recv message: {}", e)
                }
            }
        }

    }

    async fn publish_status_update(&self, event: PaymentStatusUpdatedEvent) {

        let payload = serde_json::to_string(&event).expect("Failed to serialise the evnet");


        if let Err(e) = self.producer.send(FutureRecord::to("payment-status")
                            .payload(&payload)
                            .key(&event.transaction_id.to_string()),
                            Duration::from_secs(5)).await {

                                eprintln!("Failed to publish event to kafka broker: {}", e.0);
                            }
    }
}