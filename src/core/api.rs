use axum::{extract::Query, routing::{get, post}, Router};
pub mod commands;
pub mod queries;



pub async fn create_router() -> Router {
    Router::new().nest("/api/v1", payments_routes())
}

fn payments_routes() -> Router {
    // this basically divides the api req in 2, which are then consumed by either the commnad service or the query
    Router::new()
        .nest("/transaction", transaction_routes())
        .nest("/queries", query_routes())
}

fn transaction_routes() -> Router {
    Router::new()
        .route("/", post(commands::create_transaction))

}



fn query_routes() -> Router {
    // need to figure out a way to dal with the handler taking a reference
    Router::new().route("/status/:id", get(todo!()))
}