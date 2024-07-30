use axum::{
    routing::get,
    Json, Router,
    http::header
};
use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            tower_http::cors::CorsLayer::new()
              .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
              .allow_headers(vec![header::CONTENT_TYPE])
              .allow_methods([axum::http::Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}