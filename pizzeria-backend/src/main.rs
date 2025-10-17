use std::fs;
use std::path::Path;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/transaction", post(store_transaction));

    let address = "127.0.0.1:3333";
    let listener = tokio::net::TcpListener::bind(address).await
        .expect(&format!("Failed to bind address {address}"));

    eprintln!("Server listening at {address}...");
    axum::serve(listener, app).await
        .expect("Error while starting server");
}

async fn root() -> (StatusCode, String) {
    eprintln!("Received request for pizza prebuilds.");
    let path = Path::new("pizza_prebuilds_text");
    match fs::read_to_string(path) {
        Ok(prebuilds) => {
            (StatusCode::OK, prebuilds)
        }
        Err(error) => {
            eprintln!("Error while reading file {path:?}: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, String::from(""))
        }
    }
}

async fn store_transaction(transaction_record: String) -> StatusCode {
    eprintln!("Received request to store transaction record '{transaction_record}'.");
    //todo: implement
    StatusCode::OK
}
