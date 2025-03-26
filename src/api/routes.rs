use axum::{
    routing::{get, post},
    Router,
};
use serde_json::json;

use crate::api::{
    authentication::{AuthenticationService, LoginRequest, LoginResponse},
    middleware::AuthenticatedUser,
};

pub fn create_router() -> Router {
    let auth_service = AuthenticationService::new();

    Router::new()
        .route("/login", post(login))
        .route("/protected", get(protected_route))
        .with_state(auth_service)
}

async fn login(
    axum::extract::State(auth_service): axum::extract::State<AuthenticationService>,
    axum::Json(payload): axum::Json<LoginRequest>,
) -> axum::Json<LoginResponse> {
    // In a real application, you would validate credentials against a database
    // This is just an example
    if payload.username == "admin" && payload.password == "password" {
        let token = auth_service
            .create_token("user123".to_string(), "admin".to_string())
            .expect("Failed to create token");
        
        axum::Json(LoginResponse { token })
    } else {
        panic!("Invalid credentials")
    }
}

async fn protected_route(
    AuthenticatedUser(claims): AuthenticatedUser,
) -> axum::Json<serde_json::Value> {
    axum::Json(json!({
        "message": "This is a protected route",
        "user_id": claims.sub,
        "role": claims.role
    }))
} 