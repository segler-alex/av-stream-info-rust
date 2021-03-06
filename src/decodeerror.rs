use std::error::Error;
use std::fmt;
use serde::{Deserialize, Serialize};

/// Decoding errors for headers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecodeError {
    LatMissing,
    LongMissing,
    NumberParseError,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for DecodeError {}
