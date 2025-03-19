#[cfg(test)]
mod tests {
    use std::usize;

    use axum::{
        body::{to_bytes, Body},
        http::{self, Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;

    use super::*;
    use crate::core::{api::{commands::CreateTransactionResponse, create_router}, models::TransactionStatus};

    #[tokio::test]
    async fn test_create_transaction_success() {
        // Create app
        let app = create_router().await;

        // Create test request
        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123",
            "idempotency_key": "test_key_1"
        });

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/v1/transaction")
            .header("x-idempotency-key", "test_key_1")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        // Send request
        let response = app.oneshot(request).await.unwrap();
        
        // Assert response
        assert_eq!(response.status(), StatusCode::OK);
        
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let response: CreateTransactionResponse = serde_json::from_slice(&body_bytes).unwrap();
        
        assert!(response.id.to_string().len() > 0);
        assert_eq!(response.status, TransactionStatus::Pending);
    }

    #[tokio::test]
    async fn test_create_transaction_idempotency() {
        let app = create_router().await;

        // Create test request with idempotency key
        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123",
            "idempotency_key": "test_key_2"
        });

        let make_request = || {
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/v1/transaction")
                .header("x-idempotency-key", "test_key_2")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                .unwrap();
        };

        // Send first request
        let response1 = app.clone().oneshot(make_request()).await.unwrap();
        let body1 = to_bytes(response1.into_body(), usize::MAX).await.expect("failed to parse the repsonse body");
        let response1: CreateTransactionResponse = serde_json::from_slice(&body1).unwrap();

        // Send second request with same idempotency key
        let response2 = app.oneshot(make_request()).await.unwrap();
        let body2 = to_bytes(response2.into_body(), usize::MAX).await.unwrap();
        let response2: CreateTransactionResponse = serde_json::from_slice(&body2).unwrap();

        // Both responses should have same transaction ID
        assert_eq!(response1.id, response2.id);
    }

    #[tokio::test]
    async fn test_create_transaction_missing_idempotency() {
        let app = create_router().await;

        let request_body = json!({
            "amount": 1000,
            "currency": "USD",
            "merchant_id": "merch_123",
            "customer_id": "cust_123"
        });

        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/v1/transaction")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        
        // Should return 400 Bad Request when idempotency key is missing
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
} 