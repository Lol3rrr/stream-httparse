use crate::{
    header::{HeaderKey, HeaderValue},
    Headers, StatusCode,
};

/// Represents a single HTTP-Request
#[derive(Debug, PartialEq)]
pub struct Response<'a> {
    status_code: StatusCode,
    protocol: &'a str,
    headers: Headers<'a>,
    body: Vec<u8>,
}

impl<'a> Response<'a> {
    /// Creates a new Response with the given
    /// Data as its inital State
    pub fn new(
        protocol: &'a str,
        status_code: StatusCode,
        headers: Headers<'a>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            status_code,
            protocol,
            headers,
            body,
        }
    }

    /// Serialzes the Response and returns the Data as
    /// a tuple of form (HTTP-Head, HTTP-Body)
    pub fn serialize(&self) -> (Vec<u8>, &[u8]) {
        let protocol = self.protocol;
        let status_code = self.status_code.serialize();

        let capacity = protocol.len() + 1 + status_code.len() + 4;
        let mut result = Vec::with_capacity(capacity);

        // The first line with method, path, protocol
        result.extend_from_slice(protocol.as_bytes());
        result.push(b' ');
        result.extend_from_slice(status_code.as_bytes());
        result.extend_from_slice("\r\n".as_bytes());

        // The headers
        self.headers.serialize(&mut result);

        // The ending of the head
        result.extend_from_slice("\r\n".as_bytes());

        (result, &self.body)
    }

    /// Returns the Protocol of the Response
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    /// Returns the StatusCode of the Response
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }
    /// Returns the Headers of the Response
    pub fn headers(&self) -> &Headers<'a> {
        &self.headers
    }
    /// Returns the Body of the Response
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Adds the Key-Value Pair as a new Header to
    /// the Response or replaces the old Value of the
    /// Header if it already existed on the Response
    pub fn add_header<'b, K, V>(&mut self, key: K, value: V)
    where
        'b: 'a,
        K: Into<HeaderKey<'a>>,
        V: Into<HeaderValue<'a>>,
    {
        self.headers.set(key, value);
    }

    /// Replaces the old Body of the Response with the
    /// new given Body and updates the Content-Length
    /// Header as well with the new Length
    pub fn set_body(&mut self, n_body: Vec<u8>) {
        self.body = n_body;
        self.add_header("Content-Length", self.body.len());
    }

    /// Checks if the Response is send using
    /// `Transfer-Encoding: Chunked`
    pub fn is_chunked(&self) -> bool {
        match self.headers.get("Transfer-Encoding") {
            None => false,
            Some(value) => value.eq_ignore_case(&HeaderValue::StrRef("Chunked")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_valid() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let req = Response::new(
            "HTTP/1.1",
            StatusCode::OK,
            headers,
            "body".as_bytes().to_vec(),
        );
        let raw_resp_header = "HTTP/1.1 200 OK\r\ntest-1: value-1\r\n\r\n";
        let resp_header = raw_resp_header.as_bytes().to_vec();
        let resp_body = "body".as_bytes();

        assert_eq!(req.serialize(), (resp_header, resp_body));
    }

    #[test]
    fn serialize_valid_no_body() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let req = Response::new("HTTP/1.1", StatusCode::OK, headers, "".as_bytes().to_vec());
        let raw_resp_header = "HTTP/1.1 200 OK\r\ntest-1: value-1\r\n\r\n";
        let resp_header = raw_resp_header.as_bytes().to_vec();
        let resp_body = "".as_bytes();

        assert_eq!(req.serialize(), (resp_header, resp_body));
    }

    #[test]
    fn is_chunked_not_set() {
        let mut headers = Headers::new();
        headers.set("test-1", "value-1");

        let resp = Response::new("HTTP/1.1", StatusCode::OK, headers, "".as_bytes().to_vec());

        assert_eq!(false, resp.is_chunked());
    }
    #[test]
    fn is_chunked_set() {
        let mut headers = Headers::new();
        headers.set("Transfer-Encoding", "Chunked");

        let resp = Response::new("HTTP/1.1", StatusCode::OK, headers, "".as_bytes().to_vec());

        assert_eq!(true, resp.is_chunked());
    }
    #[test]
    fn is_chunked_set_differently() {
        let mut headers = Headers::new();
        headers.set("Transfer-Encoding", "compress");

        let resp = Response::new("HTTP/1.1", StatusCode::OK, headers, "".as_bytes().to_vec());

        assert_eq!(false, resp.is_chunked());
    }
}
