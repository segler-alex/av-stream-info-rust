//! This library can analyze a http/https address and check if leads to an audio or a video stream
//! If so, then it will extract information about the stream from its metadata or in case of HLS streams
//! from its master playlist file.
//!
//! # Example
//! ```rust
//! let list = av_stream_info_rust::check("https://example.com/test.m3u", 10, 3, 3);
//! for item in list {
//!     println!("{:?}", item);
//! }
//! ```

extern crate hls_m3u8;
#[macro_use]
extern crate log;
extern crate native_tls;
extern crate playlist_decoder;
extern crate reqwest;
extern crate url;

extern crate serde;
extern crate serde_json;

//extern crate tree_magic;

mod decodeerror;
mod lat_long;
mod request;
mod streamcheck;
mod streamcheckerror;
mod streamcheckresult;
mod streamdeepscan;
mod streaminfo;

mod http_config;

use std::thread;
use std::time::Duration;

pub use decodeerror::DecodeError;
pub use http_config::extract_from_homepage;
pub use http_config::MetaInfoFile;
pub use lat_long::LatLong;
pub use streamcheckerror::StreamCheckError;
pub use streamcheckresult::StreamCheckResult;
pub use streamcheckresult::UrlType;
pub use streaminfo::StreamInfo;

/// Check url for audio/video stream.
/// # Example
/// ```rust
/// let item = av_stream_info_rust::check_tree("https://example.com/test.m3u", 10, 3, 3, true);
/// println!("{:#?}", item);
/// ```
/// # Arguments
/// * `url` - The url to check
/// * `timeout` - TCP timeout for connect and read in seconds
/// * `max_depth` - How many layers of http redirects or playlists should be followed
/// * `retries` - Retry how many times to find at least one working stream
/// * `early_exit_on_first_ok` - return from checking as early as 1 working stream has been found
pub fn check_tree(url: &str, timeout: u32, max_depth: u8, mut retries: u8, early_exit_on_first_ok: bool) -> StreamCheckResult {
    loop {
        let result = streamcheck::check(url, early_exit_on_first_ok, timeout, max_depth);
        if has_ok_result_recursive(&result) {
            return result;
        }
        if retries == 0 {
            return result;
        }

        retries -= 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn has_ok_result_recursive(result: &StreamCheckResult) -> bool {
    match &result.info {
        Ok(info) => match info {
            UrlType::Stream(_) => true,
            UrlType::Redirect(item) => has_ok_result_recursive(item),
            UrlType::PlayList(list) => {
                for item in list {
                    if has_ok_result_recursive(item) {
                        return true;
                    }
                }
                false
            }
        },
        Err(_) => false,
    }
}
