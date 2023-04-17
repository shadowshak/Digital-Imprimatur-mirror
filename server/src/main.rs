mod endpoints;
pub mod models;

use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;

use endpoints::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/user/create", post(user::create))
        .route("/user/login", post(user::login))
        .route("/user/invalidate", post(user::invalidate))
        
        .route("/sub/create", post(sub::create))
        .route("/sub/read", post(sub::read))
        .route("/sub/update", post(sub::update))
        .route("/sub/delete", post(sub::delete))
        .route("/sub/read/doc", post(sub::read_doc))
        .route("/sub/read/feedback", post(sub::read_feedback))

        .route("/doc/upload", post(doc::upload))
        .route("/doc/download", post(doc::download))
        .route("/doc/read", post(doc::read))
        .route("/doc/delete", post(doc::delete))

        .route("/feedback/upload", post(feedback::upload))
        .route("/feedback/download", post(feedback::download))
        .route("/feedback/read", post(feedback::read))
        .route("/feedback/delete", post(feedback::delete))
    ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}