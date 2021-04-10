#![allow(non_snake_case)]

use crate::DecodeError;
use crate::LatLong;

use serde::{Deserialize, Serialize};

/// Information extracted from a stream
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamInfo {
    pub Server: Option<String>,
    pub Public: Option<bool>,
    pub IceAudioInfo: Option<String>,
    pub AudioInfo: Option<String>,
    pub Name: Option<String>,
    pub Description: Option<String>,
    pub Type: String,
    pub Homepage: Option<String>,
    pub Genre: Option<String>,
    pub Bitrate: Option<u32>,
    pub Sampling: Option<u32>,
    pub CodecAudio: String,
    pub CodecVideo: Option<String>,
    pub Hls: bool,

    pub LogoUrl: Option<String>,
    pub MainStreamUrl: Option<String>,
    pub IcyVersion: u32,
    pub OverrideIndexMetaData: Option<bool>,
    pub CountryCode: Option<String>,
    pub CountrySubdivisonCode: Option<String>,
    pub LanguageCodes: Vec<String>,
    pub GeoLatLong: Option<Result<LatLong, DecodeError>>,
    pub DoNotIndex: Option<bool>,
    pub SslError: bool,
}
