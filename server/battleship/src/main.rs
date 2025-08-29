use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(hello))
        .route("/user", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8300));
    println!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    Json(user)
}

// model

use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}