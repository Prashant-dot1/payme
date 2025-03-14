use payme::core::services::payment_processor::PaymentProcessor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* for now setting it here */
    let kafka_broker = "localhost:9092";
    let stripe_secret_key = "";
    let processor = PaymentProcessor::new(kafka_broker , stripe_secret_key);

    processor.start().await
}