use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

pub fn read_pizza_prebuilds() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3333")?;

    stream.write_all(b"GET / HTTP/1.1\r
Host: 127.0.0.1:3333\r
User-Agent: curl/8.5.0\r
Accept: */*\r
\r
")?;
    stream.flush()?;

    let body = parse_http_response_body(stream)?;
    Ok(())
}

fn parse_http_response_body(stream: impl Read) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut content_length = None;
    let mut lines = reader.lines();
    // for line in lines {
    loop {
        if let Some(line) = lines.next() {
            println!("{line:?}");
            match line {
                Ok(line) => {
                    if let Some(value) = line.strip_prefix("content-length: ") {
                        let value = usize::from_str(value)
                            .unwrap();  //todo: handle error
                        content_length = Some(value);
                    } else if line.is_empty() {
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Error while reading line from HTTP response: {error}")
                }
            }
        } else { break }
    }
    if let Some(content_length) = content_length {
        // let mut buffer = vec![content_length];
        // reader.read_exact(&mut buffer);
        let body: Result<String, Error> = lines.collect();
        println!("{body:?}");
        Ok(body?)
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
