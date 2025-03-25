
mod core;
mod api;

use axum::{
    middleware,
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

use crate::api::{
    middleware::AuthMiddleware,
    routes::create_router,
};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create the router with authentication
    let app = create_router()
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            AuthMiddleware::new(),
            |state, req, next| async move {
                let mut auth_middleware = state.clone();
                auth_middleware.authorize(req).await?;
                next.run(req).await
            },
        ));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
