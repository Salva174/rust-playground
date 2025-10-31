pub mod request;

use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use crate::Arguments;
use crate::error::FrontendError;


use crate::http::request::RequestBuilder;

pub fn read_pizza_prebuilds(arguments: &Arguments) -> io::Result<String> {
    let mut stream = TcpStream::connect(arguments.server_address)?;

    let request = RequestBuilder::get()
        .host(arguments.server_address.to_string())
        .build();

    write!(stream, "{}", request)?;
    stream.flush()?;

    let body = parse_http_response_body(stream)
        .map_err(FrontendError::into_io)?;
    Ok(body)
}

pub fn read_toppings(arguments: &Arguments) -> io::Result<String> {
    let mut stream = TcpStream::connect(arguments.server_address)?;

    let request = RequestBuilder::get()
        .host(arguments.server_address.to_string())
        .path(String::from("/toppings"))
        .build();

    write!(stream, "{}", request)?;
    stream.flush()?;

    let body = parse_http_response_body(stream)
        .map_err(FrontendError::into_io)?;
    Ok(body)
}

pub fn send_transaction_record(transaction_record: String, arguments: &Arguments) -> io::Result<()> {
    let mut stream = TcpStream::connect(arguments.server_address)?;
    let transaction_record_length = transaction_record.len();

    let request = RequestBuilder::post()
        .path(String::from("/transaction"))
        .host(arguments.server_address.to_string())
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

        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length") {
                let len = value.trim().parse::<usize>()
                    .map_err(|error| FrontendError::InvalidContentLength {
                        value: value.trim().to_string(),
                        source: error,
                    })?;
                content_length = Some(len);
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
    use std::io::Cursor;

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
}
