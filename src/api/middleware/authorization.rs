use std::pin::Pin;
use http::request::Request as MyRequest;
use axum::{
    body::Bytes,
    extract::FromRequestParts,
    response::Response,
    RequestPartsExt,
};
use tower::ServiceExt;
use tower_http::auth::AuthorizeRequest;
use http_body::Body as HttpBody;

use crate::api::authentication::{AuthenticationError, AuthenticationService, Claims};

#[derive(Clone)]
pub struct AuthMiddleware {
    auth_service: AuthenticationService,
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self {
            auth_service: AuthenticationService::new(),
        }
    }
}

impl AuthorizeRequest for AuthMiddleware {
    type Output = ();
    type ResponseBody = Pin<Box<dyn HttpBody<Data = Bytes, Error = AuthenticationError> + Send + 'static>>;

    fn unauthorized_response<B>(&mut self, _request: &MyRequest<B>) -> Response<Self::ResponseBody> {
        // let body = Box::pin(http_body_util::Empty::new());
        // Response::builder()
        //     .status(StatusCode::UNAUTHORIZED)
        //     .body(body)
        //     .unwrap()
        todo!("this doesn't seem to work ");
    }

    fn authorize<B>(&mut self, request: &MyRequest<B>) -> Option<Self::Output> {
        let (mut parts, body) = request.into_parts();
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| self.unauthorized_response(request))?;

        if !auth_header.starts_with("Bearer ") {
            self.unauthorized_response(request);
            return None;
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let claims = self.auth_service.validate_token(token)
            .ok()?;

        parts.extensions.insert(claims);
        *request = Request::from_parts(parts, body);

        Some(())
    }
}

pub struct AuthenticatedUser(pub Claims);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthenticationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or(AuthenticationError::InvalidToken)?;

        Ok(AuthenticatedUser(claims))
    }
} 