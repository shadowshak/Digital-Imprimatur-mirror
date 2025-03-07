mod endpoints;
pub mod models;
pub mod controllers;

use axum::{
    Router,
    routing::{get, post}, http::Method,
};
use tower_http::cors::{CorsLayer, AllowOrigin, Any};
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
        .route("/user/change_password", post(user::change_password))
        .route("/user/get_info", post(user::get_info))
        .route("/user/submissions", post(user::submissions))
        
        .route("/sub/create", post(sub::create))
        .route("/sub/read", post(sub::read))
        .route("/sub/update", post(sub::update))
        .route("/sub/delete", post(sub::delete))
        .route("/sub/read/document", post(sub::read_doc))
        .route("/sub/read/feedback", post(sub::read_feedback))

        .route("/document/upload", post(doc::upload))
        .route("/document/download", post(doc::download))
        .route("/document/read", post(doc::read))
        .route("/document/delete", post(doc::delete))

        .route("/feedback/upload", post(feedback::upload))
        .route("/feedback/download", post(feedback::download))
        .route("/feedback/read", post(feedback::read))
        .route("/feedback/delete", post(feedback::delete))

        .layer(CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(|_, _| true))
            .allow_headers(Any)
            .allow_methods([Method::GET]))
    ;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}