use crate::{general::StringContainer, header::HeaderValue, Headers, Method};

/// Represents a single HTTP-Request
#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    method: Method,
    path: StringContainer<'a>,
    protocol: &'a str,
    headers: Headers<'a>,
    body: &'a [u8],
}

impl<'a> Request<'a> {
    /// Creates a new Request with the given Data as its
    /// initial Data
    pub fn new(
        protocol: &'a str,
        method: Method,
        path: &'a str,
        headers: Headers<'a>,
        body: &'a [u8],
    ) -> Self {
        Self {
            method,
            path: StringContainer::Ref(path),
            protocol,
            headers,
            body,
        }
    }

    /// Serializes the Request and returns the final Data
    /// as a tuple of (HTTP-Head, HTTP-Body)
    pub fn serialize(&self) -> (Vec<u8>, &[u8]) {
        let method = self.method.serialize();
        let path = self.path.as_ref();
        let capacity = method.len() + 1 + path.len() + 1 + self.protocol.len() + 4;
        let mut result = Vec::with_capacity(capacity);

        // The first line with method, path, protocol
        result.extend_from_slice(method.as_bytes());
        result.push(b' ');
        result.extend_from_slice(path.as_bytes());
        result.push(b' ');
        result.extend_from_slice(self.protocol.as_bytes());
        result.extend_from_slice("\r\n".as_bytes());

        // The headers
        self.headers.serialize(&mut result);

        // The ending of the head
        result.extend_from_slice("\r\n".as_bytes());

        (result, self.body)
    }

    /// Returns the Protocol of the Request
    pub fn protocol(&self) -> &'a str {
        &self.protocol
    }
    /// Returns the Method of the Request
    pub fn method(&self) -> &Method {
        &self.method
    }
    /// Returns the Path of the Request
    pub fn path(&'a self) -> &'a str {
        self.path.as_ref()
    }
    /// Returns the Headers of the Request
    pub fn headers(&self) -> &Headers<'a> {
        &self.headers
    }
    /// Returns a mutable Reference to the Headers of the Request
    pub fn header_mut(&mut self) -> &mut Headers<'a> {
        &mut self.headers
    }
    /// Returns the Body of the Request
    pub fn body(&self) -> &[u8] {
        self.body
    }

    /// Checks if the Requests expects a
    /// Keep-alive connection
    pub fn is_keep_alive(&self) -> bool {
        match self.headers.get("Connection") {
            None => false,
            Some(value) => value.eq_ignore_case(&HeaderValue::StrRef("Keep-Alive")),
        }
    }

    /// Overwrites the Path with the new Path
    pub fn set_path_ref<'b>(&mut self, n_path: &'b str)
    where
        'b: 'a,
    {
        self.path = StringContainer::Ref(n_path);
    }
    /// Overwrites the Path with the new Path, but using
    /// an owned String instead of a reference
    pub fn set_path_owned(&mut self, n_path: String) {
        self.path = StringContainer::Owned(n_path);
    }
}

impl std::fmt::Display for Request<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] Path: '{}'", self.method, self.path.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_path_to_own() {
        let mut req = Request::new("HTTP/1.1", Method::GET, "/test/path", Headers::new(), &[]);

        let path = req.path().to_owned();
        req.set_path_ref(&path[1..]);

        assert_eq!("test/path", req.path());
    }

    #[test]
    fn serialize_valid() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let req = Request::new("HTTP/1.1", Method::GET, "/test", headers, "body".as_bytes());
        let raw_header = "GET /test HTTP/1.1\r\ntest-1: value-1\r\n\r\n";
        let header_resp = raw_header.as_bytes().to_vec();
        let body_resp = "body".as_bytes();

        assert_eq!(req.serialize(), (header_resp, body_resp));
    }
    #[test]
    fn serialize_valid_no_body() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let req = Request::new("HTTP/1.1", Method::GET, "/test", headers, "".as_bytes());
        let raw_header = "GET /test HTTP/1.1\r\ntest-1: value-1\r\n\r\n";
        let resp_header = raw_header.as_bytes().to_vec();
        let resp_body = "".as_bytes();

        assert_eq!(req.serialize(), (resp_header, resp_body));
    }

    #[test]
    fn is_keep_alive_not_set() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let req = Request::new("HTTP/1.1", Method::GET, "/test", headers, "".as_bytes());

        assert_eq!(false, req.is_keep_alive());
    }
    #[test]
    fn is_keep_alive_is_set() {
        let mut headers = Headers::new();
        headers.set("Connection", "Keep-Alive");

        let req = Request::new("HTTP/1.1", Method::GET, "/test", headers, "".as_bytes());

        assert_eq!(true, req.is_keep_alive());
    }
    #[test]
    fn is_keep_alive_is_set_to_off() {
        let mut headers = Headers::new();
        headers.set("Connection", "Close");

        let req = Request::new("HTTP/1.1", Method::GET, "/test", headers, "".as_bytes());

        assert_eq!(false, req.is_keep_alive());
    }
}
