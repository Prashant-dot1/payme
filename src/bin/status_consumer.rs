use payme::core::services::status_consumer::StatusConsumer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kafka_broker = "localhost:9092";
    
    let consumer = StatusConsumer::new(kafka_broker);
    consumer.start().await
}