use crate::StreamCheckError;
use crate::StreamInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UrlType {
    Stream(StreamInfo),
    Redirect(Box<StreamCheckResult>),
    PlayList(Vec<StreamCheckResult>),
}

/// A check result for a single url
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamCheckResult {
    url: String,
    pub info: Result<UrlType, StreamCheckError>,
}

impl StreamCheckResult {
    pub fn new(url: &str, info: Result<UrlType, StreamCheckError>) -> Self {
        StreamCheckResult {
            url: url.to_string(),
            info,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}
