use tokio::fs;
use std::path::Path;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

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
    match fs::read_to_string(path).await {
        Ok(prebuilds) => {
            (StatusCode::OK, prebuilds)
        }
        Err(error) => {
            eprintln!("Error while reading file {path:?}: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, String::from(""))
        }
    }
}

const LOG_PATH: &str = "transactions.log";

async fn store_transaction(mut transaction_record: String) -> StatusCode {
    eprintln!("Received request to store transaction record '{transaction_record}'.");

    if !transaction_record.ends_with('\n') {
        transaction_record.push('\n');
    }

    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .await
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(transaction_record.as_bytes()).await {
                eprintln!("write error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            } else {
                StatusCode::NO_CONTENT
            }
        }
        Err(e) => {
            eprintln!("open error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
