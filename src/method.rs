/// The different HTTP-Methods as defined by
/// [RFC 2616 5.1.1](https://tools.ietf.org/html/rfc2616#section-5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    /// Requests the Communication-Options available
    /// for a given Ressource
    OPTIONS,
    /// Retrieves the specified Ressource from the Server
    GET,
    /// Identical to the GET-Method, but the Server is not
    /// required to return a Response-Body
    HEAD,
    /// Used to post Data to the Server
    POST,
    /// Tells the Server to store the supplied Body under a
    /// given Ressource URI
    PUT,
    /// Requests that the given Data assosicated with the
    /// Ressource-URI is deleted
    DELETE,
    /// Used to invoke a remote application-layer loopback
    TRACE,
    /// Reserved
    CONNECT,
}

impl Method {
    /// Parses the raw Method into one of the known Methods,
    /// returns None if the Method is unknown
    pub fn parse(raw_method: &str) -> Option<Method> {
        match raw_method {
            "OPTIONS" => Some(Method::OPTIONS),
            "GET" => Some(Method::GET),
            "HEAD" => Some(Method::HEAD),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "TRACE" => Some(Method::TRACE),
            "CONNECT" => Some(Method::CONNECT),
            _ => None,
        }
    }

    /// Serializes the Method into a static String
    /// for that Method
    pub fn serialize(&self) -> &'static str {
        match *self {
            Method::OPTIONS => "OPTIONS",
            Method::GET => "GET",
            Method::HEAD => "HEAD",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_method_options() {
        assert_eq!(Some(Method::OPTIONS), Method::parse("OPTIONS"));
    }
    #[test]
    fn parse_method_get() {
        assert_eq!(Some(Method::GET), Method::parse("GET"));
    }
    #[test]
    fn parse_method_head() {
        assert_eq!(Some(Method::HEAD), Method::parse("HEAD"));
    }
    #[test]
    fn parse_method_post() {
        assert_eq!(Some(Method::POST), Method::parse("POST"));
    }
    #[test]
    fn parse_method_put() {
        assert_eq!(Some(Method::PUT), Method::parse("PUT"));
    }
    #[test]
    fn parse_method_delete() {
        assert_eq!(Some(Method::DELETE), Method::parse("DELETE"));
    }
    #[test]
    fn parse_method_trace() {
        assert_eq!(Some(Method::TRACE), Method::parse("TRACE"));
    }
    #[test]
    fn parse_method_connect() {
        assert_eq!(Some(Method::CONNECT), Method::parse("CONNECT"));
    }
    #[test]
    fn parse_method_invalid() {
        assert_eq!(None, Method::parse("DIFFERENT"));
    }
}
