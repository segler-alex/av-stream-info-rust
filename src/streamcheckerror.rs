use std::error::Error;
use std::fmt;

/// Posible errors for stream checking.
/// First parameter is the url.
#[derive(Debug, Clone)]
pub enum StreamCheckError {
    ConnectionFailed(),
    IllegalStatusCode(u32),
    MaxDepthReached(),
    MissingContentType(),
    PlayListDecodeError(),
    PlaylistEmpty(),
    PlaylistReadFailed(),
    UnknownContentType(String),
    UrlJoinError(),
    UrlParseError(),
}

impl fmt::Display for StreamCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StreamCheckError::ConnectionFailed() => write!(f, "ConnectionFailed()"),
            StreamCheckError::IllegalStatusCode(code) => write!(f, "IllegalStatusCode({})", code),
            StreamCheckError::MaxDepthReached() => write!(f, "MaxDepthReached()"),
            StreamCheckError::MissingContentType() => write!(f, "MissingContentType()"),
            StreamCheckError::PlayListDecodeError() => write!(f, "PlayListDecodeError()"),
            StreamCheckError::PlaylistEmpty() => write!(f, "PlaylistEmpty()"),
            StreamCheckError::PlaylistReadFailed() => write!(f, "PlaylistReadFailed()"),
            StreamCheckError::UnknownContentType(content_type) => write!(f, "UnknownContentType({})", content_type),
            StreamCheckError::UrlJoinError() => write!(f, "UrlJoinError()"),
            StreamCheckError::UrlParseError() => write!(f, "UrlParseError()"),
        }
    }
}

impl Error for StreamCheckError {}
