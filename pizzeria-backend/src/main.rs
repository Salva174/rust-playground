use tokio::fs;
use std::path::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{delete, get, post};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use serde::Deserialize;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/transaction", post(store_transaction))
        .route("/toppings", get(get_toppings).post(add_topping).delete(delete_topping));

    let address = "127.0.0.1:3333";
    let listener = tokio::net::TcpListener::bind(address).await
        .expect(&format!("Failed to bind address {address}"));

    eprintln!("Server listening at {address}...");
    axum::serve(listener, app).await
        .expect("Error while starting server");
}

async fn root() -> (StatusCode, String) {
    eprintln!("Received request for Order Menu.");
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

async fn get_toppings() -> (StatusCode, String) {
    eprintln!("Received request for Topping List.");
    let path = Path::new("toppings_text");

    match fs::read_to_string(path).await {
        Ok(toppings) => {
            (StatusCode::OK, toppings)
        }
        Err(err) => {
            eprintln!("Error while reading file {path:?} {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, String::new())
        }
    }
}

async fn add_topping(body: String) -> StatusCode {
    let mut line = body;
    if !line.ends_with('\n') { line.push('\n');
    }

    match OpenOptions::new().create(true).append(true).open("toppings_text").await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(line.as_bytes()).await {
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

#[derive(Deserialize)]
struct DeleteParameters { name : String }

async fn delete_topping(Query(p): Query<DeleteParameters>) -> StatusCode {
    eprintln!("DELETE /toppings?name={}", p.name);
    match fs::read_to_string("toppings_text").await {
        Ok(content) => {
            let mut kept = String::new();
            for line in content.lines() {
                let name = line.split('#').next().unwrap_or("");
                if !name.eq_ignore_ascii_case(&p.name) {
                    kept.push_str(line);
                    kept.push('\n');
                }
            }
            if let Err(e) = fs::write("toppings_text", kept).await {
                eprintln!("write error: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            StatusCode::NO_CONTENT
        }
        Err(e) => {
            eprintln!("read error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


//todo: - nach löschen Ausgabe fixen
//      - Log Ausgabe für Toppings hinzufügen (bei Add/Del)
//      - Backend-Ausgabe korrigieren ("Order Menu")
