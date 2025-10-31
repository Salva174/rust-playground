pub struct RequestBuilder {
    method: &'static str,
    path: Option<String>,
    host: Option<String>,
    content_type: Option<String>,
    content_length: Option<usize>,
    body: Option<String>,
}

impl RequestBuilder {

    fn new(method: &'static str) -> Self {
        Self {
            method,
            path: None,
            host: None,
            content_type: None,
            content_length: None,
            body: None,
        }
    }

    pub fn get() -> RequestBuilder {
        Self::new("GET")
    }

    pub fn post() -> RequestBuilder {
        Self::new("POST")
    }

    pub fn delete() -> RequestBuilder {
        Self::new("DELETE")
    }

    pub fn path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = Some(host);
        self
    }

    pub fn content_type(&mut self, content_type: String) -> &mut Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_length(&mut self, content_length: usize) -> &mut Self {
        self.content_length = Some(content_length);
        self
    }


    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(&self) -> String {
        let method = self.method;
        let path = self.path.as_ref().map(String::as_str).unwrap_or_else(|| "/");
        let host = self.host.as_ref().expect("host should be specified.");


        let mut request = format!("{method} {path} HTTP/1.1\r
Host: {host}\r
");
        if let Some(content_type) = self.content_type.as_ref() {
            request.push_str(&format!("Content-Type: {content_type}\r\n"))
        }

        if let Some(content_length) = self.content_length.as_ref() {
            request.push_str(&format!("Content-Length: {content_length}\r\n"))
        }

        request.push_str("Connection: close\r\n");

        request.push_str("\r\n");

        if let Some(body) = self.body.as_ref() {
            request.push_str(&format!("{body}"))
        }

        request
    }
}

#[cfg(test)]
mod tests {
    use crate::http::request::RequestBuilder;

    #[test]
    fn test() {

        let request = RequestBuilder::post()
            .path(String::from("/foobar"))
            .host(String::from("1.2.3.4:3333"))
            .content_type(String::from("text/plain; charset=utf-8"))
            .content_length(125)
            .body(String::from("ABCDEF"))
            .build();

        let expected_request = "POST /foobar HTTP/1.1\r
Host: 1.2.3.4:3333\r
Content-Type: text/plain; charset=utf-8\r
Content-Length: 125\r
Connection: close\r
\r
ABCDEF";
        assert_eq!(request, expected_request)

    }

    #[test]
    fn test_that_default_path_is_root() {

        let request = RequestBuilder::post()
            .host(String::from("1.2.3.4:3333"))
            .build();

        let expected_request = "POST / HTTP/1.1\r
Host: 1.2.3.4:3333\r
Connection: close\r
\r
";
        assert_eq!(request, expected_request)

    }

    #[test]
    fn test_that_default_method_is_get() {

        let request = RequestBuilder::get()
            .path(String::from("/foobar"))
            .host(String::from("1.2.3.4:3333"))
            .build();

        let expected_request = "GET /foobar HTTP/1.1\r
Host: 1.2.3.4:3333\r
Connection: close\r
\r
";
        assert_eq!(request, expected_request)

    }

    #[test]
    #[should_panic]
    fn test_that_builder_panics_with_no_host() {

        let request = RequestBuilder::post()
            .path(String::from("/foobar"))
            .build();

        let expected_request = "POST /foobar HTTP/1.1\r
Host: 1.2.3.4:3333\r
Connection: close\r
\r
";
        assert_eq!(request, expected_request)

    }
}
