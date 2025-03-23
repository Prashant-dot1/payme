#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use hyper::StatusCode;
    use serde_json::json;

    use super::*;
    use crate::core::{api::{commands::CreateTransactionResponse, create_router}, models::TransactionStatus};

    #[tokio::test]
    async fn test_create_transaction_success() {
        let app = create_router().await;
        let server = TestServer::new(app).unwrap();

        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123",
            "idempotency_key": "test_key_1"
        });

        let response = server
            .post("/api/v1/transaction")
            .add_header("x-idempotency-key", "test_key_1")
            .json(&request_body)
            .await;

        response.assert_status_ok();
        
        let response_body = response.json::<CreateTransactionResponse>();
        assert!(response_body.id.to_string().len() > 0);
        assert_eq!(response_body.status, TransactionStatus::Pending);
    }

    #[tokio::test]
    async fn test_create_transaction_idempotency() {
        let app = create_router().await;
        let server = TestServer::new(app).unwrap();

        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123",
            "idempotency_key": "test_key_2"
        });

        // First request
        let response1 = server
            .post("/api/v1/transaction")
            .add_header("x-idempotency-key", "test_key_2")
            .json(&request_body)
            .await;
        let body1 = response1.json::<CreateTransactionResponse>();

        // Second request with same idempotency key
        let response2 = server
            .post("/api/v1/transaction")
            .add_header("x-idempotency-key", "test_key_2")
            .json(&request_body)
            .await;
        let body2 = response2.json::<CreateTransactionResponse>();

        assert_eq!(body1.id, body2.id);
    }

    #[tokio::test]
    async fn test_create_transaction_missing_idempotency() {
        let app = create_router().await;
        let server = TestServer::new(app).unwrap();

        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123"
        });

        let response = server
            .post("/api/v1/transaction")
            .json(&request_body)
            .await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }
} 