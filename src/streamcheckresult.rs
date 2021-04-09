use crate::StreamCheckError;
use crate::StreamInfo;

/// A check result for a single url
pub struct StreamCheckResult {
    url: String,
    pub info: Result<StreamInfo, StreamCheckError>,
}

impl StreamCheckResult {
    pub fn new(url: &str, info: Result<StreamInfo, StreamCheckError>) -> Self {
        StreamCheckResult {
            url: url.to_string(),
            info,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}