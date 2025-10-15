use axum::{Router, routing::get};
use database::envs;

#[tokio::main]
async fn main() {
    // build app and add single route
    let app = Router::new().route("/", get(|| async { "Hello World!" }));

    let address = envs::db_address();
    let port = envs::db_port();
    println!("Now Listening on {address}:{port}...");
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
