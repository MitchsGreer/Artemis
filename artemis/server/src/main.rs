mod api;

use database::envs;
use api::openapi::update_api_spec;


#[tokio::main]
async fn main() {
    update_api_spec();

    let app = api::create_app();

    let address = envs::db_address();
    let port = envs::db_port();
    println!("Now Listening on {address}:{port}...");
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
