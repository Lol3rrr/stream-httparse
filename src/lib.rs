#![warn(missing_docs)]
//! A fast and simple to use HTTP-Parsing crate

mod request;
pub use request::Request;

mod response;
pub use response::Response;

mod status_code;
pub use status_code::StatusCode;

mod method;
pub use method::Method;

/// Holds some more Types that are needed for Headers
pub mod header;
pub use header::Header;

mod headers;
pub use headers::Headers;

mod chunk;
pub use chunk::Chunk;

pub(crate) mod general;

/// This module holds all the Parsers that can deal
/// with parsing the Data in multiple chunks and dont
/// need all of it right away
pub mod streaming_parser;
