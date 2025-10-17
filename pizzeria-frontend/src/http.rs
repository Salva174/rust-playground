use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

pub fn read_pizza_prebuilds() -> io::Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;

    stream.write_all(b"GET / HTTP/1.1\r
Host: 127.0.0.1:3333\r
User-Agent: curl/8.5.0\r
Accept: */*\r
\r
")?;
    stream.flush()?;

    let body = parse_http_response_body(stream)?;
    Ok(body)
}

pub fn send_transaction_record(transaction_record: String) -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;
    let transaction_record_length = transaction_record.len();

    stream.write_all(format!("POST /transaction HTTP/1.1\r
Host: 127.0.0.1:3333\r
content-type: text/plain; charset=utf-8\r
content-length: {transaction_record_length}\r
\r
{transaction_record}").as_bytes())?;
    stream.flush()?;

    let _ = parse_http_response_body(stream)?; //receive empty body to avoid connection closing before server responded

    //todo: validate response status code is successful

    Ok(())
}

fn parse_http_response_body(stream: impl Read) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut content_length = None;
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let read_bytes = reader.read_line(&mut buffer)?;
        if read_bytes > 0 {
            let line = &buffer.trim();
            println!("{line:?}");
            if let Some(value) = line.strip_prefix("content-length: ") {
                let value = usize::from_str(value)
                    .unwrap();  //todo: handle error
                content_length = Some(value);
            } else if line.is_empty() {
                break;
            }
        } else { break }
    }
    if let Some(content_length) = content_length {
        let mut buffer = vec![0; content_length];
        reader.read_exact(&mut buffer)?;
        let body = String::from_utf8(buffer).unwrap();
        println!("{body:?}");
        Ok(body)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "No content length header in server response, while reading pizza prebuilds."))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

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
