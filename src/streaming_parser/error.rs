/// A simple Result wrapper that already has the ParseError
/// type as its Error-Type
pub type ParseResult<T> = Result<T, ParseError>;

/// Indicates all Errors related to parsing a Request/Response/Chunk
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Could not find a valid Method in the Request
    MissingMethod,
    /// Could not find a valid Path in the Request
    MissingPath,
    /// Could not identify the Protocol of the Request/Response
    MissingProtocol,
    /// Could not find any headers in the Request/Response
    MissingHeaders,
    /// Could not find a StatusCode in the Response
    MissingStatusCode,
    /// Returned StatusCode is not valid
    InvalidStatusCode,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::MissingMethod => write!(f, "Missing Method"),
            Self::MissingPath => write!(f, "Missing Path"),
            Self::MissingProtocol => write!(f, "Missing Protocol"),
            Self::MissingHeaders => write!(f, "Missing Headers"),
            Self::MissingStatusCode => write!(f, "Missing StatusCode"),
            Self::InvalidStatusCode => write!(f, "Invalid StatusCode"),
        }
    }
}
