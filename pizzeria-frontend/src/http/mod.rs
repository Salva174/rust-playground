mod address;
pub mod request;

use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use crate::error::FrontendError;

pub use address::{
    backend_socket_addr,
    BACKEND_HOST_KEY,
    BACKEND_PORT_KEY
};
use crate::http::request::RequestBuilder;

pub fn read_pizza_prebuilds() -> io::Result<String> {
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream = TcpStream::connect(addr)?;

    let request = RequestBuilder::get()
        .host(addr.to_string())
        .build();

    write!(stream, "{}", request)?;
    stream.flush()?;

    let body = parse_http_response_body(stream)
        .map_err(FrontendError::into_io)?;
    Ok(body)
}

pub fn read_toppings() -> io::Result<String> {
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream = TcpStream::connect(addr)?;

    let request = RequestBuilder::get()
        .host(addr.to_string())
        .path(String::from("/toppings"))
        .build();

    write!(stream, "{}", request)?;
    stream.flush()?;

    let body = parse_http_response_body(stream)
        .map_err(FrontendError::into_io)?;
    Ok(body)
}

pub fn send_transaction_record(transaction_record: String) -> io::Result<()> {
    let addr = backend_socket_addr()
        .map_err(FrontendError::into_io)?;
    let mut stream = TcpStream::connect(addr)?;
    let transaction_record_length = transaction_record.len();

    let request = RequestBuilder::post()
        .path(String::from("/transaction"))
        .host(addr.to_string())
        .content_type(String::from("text/plain; charset=utf-8"))
        .content_length(transaction_record_length)
        .body(transaction_record)
        .build();

    stream.write_all(request.as_bytes())?;

    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let mut status_line = String::new();
    reader.read_line(&mut status_line)?;
    let code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|str| str.parse::<u16>().ok())
        .unwrap_or(0);

    if !(200..300).contains(&code) {
        return Err(
            FrontendError::HttpStatus { code }.into_io(),
        );
    }

    let _ = parse_http_response_body(reader)
        .map_err(FrontendError::into_io)?; //receive empty body to avoid connection closing before server responded

    //todo: validate response status code is successful

    Ok(())
}

fn parse_http_response_body(stream: impl Read) -> Result<String, FrontendError> {
    let mut reader = BufReader::new(stream);
    let mut content_length: Option<usize> = None;
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let read_bytes = reader.read_line(&mut buffer)
            .map_err(|_| FrontendError::UnexpectedEof)?;
        if read_bytes == 0 {
            break;      //Verbindung zu
        }
        let line = &buffer.trim();
        // println!("{line:?}");

        if line.is_empty() {
            break;      //Header ende
        }

        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                let len = value.trim().parse::<usize>()
                    .map_err(|error| FrontendError::InvalidContentLength {
                        value: value.trim().to_string(),
                        source: error,
                    })?;
                content_length = Some(len);
            }
        }
    }

    let n = content_length.unwrap_or(0);

    if n == 0 {
        // Kein Body -> leeren String zurückgeben
        return Ok(String::new());
    }

    let mut body_buffer = vec![0u8; n];
    reader.read_exact(&mut body_buffer)
        .map_err(|_| FrontendError::UnexpectedEof)?;
    let body = String::from_utf8(body_buffer)
        .map_err(|error| FrontendError::BodyUtf8 {source: error})?;
    // println!("{body:?}");
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::append_line_sync;
    use crate::transactions::format_transaction_as_string;
    use std::io::Cursor;
    use std::{env, fs};
    use tempfile::tempdir;

    #[test]
    fn test_parse_http_response_body() -> Result<(), Box<dyn std::error::Error>> {

        let input = Cursor::new("HTTP/1.1 200 OK\r
content-type: text/plain; charset=utf-8\r
content-length: 9\r
date: Fri, 17 Oct 2025 07:50:31 GMT\r
\r
body-text");

        let result = parse_http_response_body(input)?;
        assert_eq!(result, String::from("body-text"));

        Ok(())
    }

    #[test]
    fn fallback_writes_log_when_backend_is_down() {
        // 1) isoliertes Arbeitsverzeichnis
        let tmp = tempdir().unwrap();
        let old = env::current_dir().unwrap();
        env::set_current_dir(tmp.path()).unwrap();
        const LOG_PATH: &str = "transactions.log";


        // 2) Transaktionszeile bauen
        let mut line = format_transaction_as_string(700, "TestPizza");
        if !line.ends_with('\n') { line.push('\n'); }

        // 3) Backend absichtlich NICHT starten => send schlägt fehl
        if let Err(_e) = send_transaction_record(line.clone()) {
            // 4) Fallback: direkt in Datei schreiben 
            append_line_sync(LOG_PATH, &line).expect("fallback write failed");
        } else {
            panic!("Expected backend error, but request succeeded");
        }

        // 5) prüfen, dass Log geschrieben wurde
        let content = fs::read_to_string(LOG_PATH).unwrap();
        assert!(content.contains("TestPizza"));

        // 6) CWD zurück
        env::set_current_dir(old).unwrap();
    }
}
