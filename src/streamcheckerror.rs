use std::error::Error;
use std::fmt;

#[derive(Debug,Clone)]
pub struct StreamCheckError {
    pub url: String,
    pub msg: String,
}

impl StreamCheckError {
    pub fn new(url: &str, msg: &str) -> StreamCheckError {
        StreamCheckError {
            url: url.to_string(),
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for StreamCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for StreamCheckError {
    fn description(&self) -> &str {
        &self.msg
    }
}