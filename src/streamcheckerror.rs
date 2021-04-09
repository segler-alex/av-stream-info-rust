use std::error::Error;
use std::fmt;

/// Posible errors for stream checking.
/// First parameter is the url.
#[derive(Debug, Clone)]
pub enum StreamCheckError {
    ConnectionFailed(String),
    IllegalStatusCode(String, u32),
    MaxDepthReached(String),
    MissingContentType(String),
    PlayListDecodeError(String),
    PlaylistEmpty(String),
    PlaylistReadFailed(String),
    UnknownContentType(String, String),
    UrlJoinError(String),
    UrlParseError(String),
}

impl fmt::Display for StreamCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamCheckError::ConnectionFailed(url) => write!(f, "ConnectionFailed({})", url),
            StreamCheckError::IllegalStatusCode(url, code) => write!(f, "IllegalStatusCode({}, {})", url, code),
            StreamCheckError::MaxDepthReached(url) => write!(f, "MaxDepthReached({})", url),
            StreamCheckError::MissingContentType(url) => write!(f, "MissingContentType({})", url),
            StreamCheckError::PlayListDecodeError(url) => write!(f, "PlayListDecodeError({})", url),
            StreamCheckError::PlaylistEmpty(url) => write!(f, "PlaylistEmpty({})", url),
            StreamCheckError::PlaylistReadFailed(url) => write!(f, "PlaylistReadFailed({})", url),
            StreamCheckError::UnknownContentType(url, content_type) => write!(f, "UnknownContentType({}, {})", url, content_type),
            StreamCheckError::UrlJoinError(url) => write!(f, "UrlJoinError({})", url),
            StreamCheckError::UrlParseError(url) => write!(f, "UrlParseError({})", url),
        }
    }
}

impl Error for StreamCheckError {}
