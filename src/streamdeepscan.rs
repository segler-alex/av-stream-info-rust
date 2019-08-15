use tree_magic;

use log::{debug};

pub struct ScanResult{
    pub mime: String
}

pub fn scan(bytes: &[u8]) -> Result<Option<ScanResult>, Box<std::error::Error>> {
    let mime = tree_magic::from_u8(bytes);
    debug!("found mime type: {}", mime);
    if mime != "application/octet-stream" {
        Ok(Some(ScanResult{mime}))
    }else{
        Ok(None)
    }
}