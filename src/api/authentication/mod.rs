use std::env;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse{
    pub token: String
}

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Invalid Token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Missing Token")]
    MissingToken,
    #[error("Invalid credentials")]
    InvalidCredentials
}   

#[derive(Clone)]
pub struct AuthenticationService {
    encoding_key: EncodingKey,
    decoding_key : DecodingKey
}

impl AuthenticationService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("jwt secet must be set");
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes())
        }
    }

    pub fn create_token(&self, user_id: String, role: String) -> Result<String, AuthenticationError> {

        let expiry = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::hours(24))
                    .expect("valid timestamp")
                    .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: expiry,
            role
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthenticationError::InvalidToken)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthenticationError> {
        let validation = Validation::default();
        decode(token, &self.decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|_| AuthenticationError::InvalidToken)
    }
}



impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthenticationError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthenticationError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthenticationError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AuthenticationError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
        };

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
