use axum::{
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    response::Response,
    RequestPartsExt,
};
use tower::ServiceExt;
use tower_http::auth::AuthorizeRequest;
use http_body::{Body, Empty};
// use http_body_util::combinators::box_body::BoxBody;

use crate::api::authentication::{AuthenticationError, AuthenticationService, Claims, LoginRequest};

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
    type ResponseBody = Box<dyn Body>;

    fn unauthorized_response<B>(&mut self, request: &Request<B>) -> Response<Self::ResponseBody> {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Box::new(Body::new(Empty::new())))
            .unwrap()
    }

    fn authorize<B>(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let (mut parts, _) = request.into_parts();
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| self.unauthorized_response(request))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(self.unauthorized_response(request));
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let claims = self.auth_service.validate_token(token)
            .map_err(|_| self.unauthorized_response(request))?;

        parts.extensions.insert(claims);
        *request = Request::from_parts(parts, ());

        Ok(())
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