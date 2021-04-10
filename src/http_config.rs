use crate::DecodeError;
use crate::LatLong;
use std::convert::TryFrom;
use serde::de::{self, Deserializer, Unexpected};
use std::error::Error;
use serde::{Deserialize, Serialize};

use reqwest::blocking::get;

/// Does contain decoded information from a stream information file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaInfoFile {
    #[serde(rename = "icy-index-metadata")]
    #[serde(deserialize_with = "bool_from_int")]
    pub index_metadata: bool,
    #[serde(rename = "icy-version")]
    pub version: u8,
    #[serde(rename = "icy-main-stream-url")]
    pub main_stream_url: Option<String>,
    #[serde(rename = "icy-name")]
    pub name: Option<String>,
    #[serde(rename = "icy-description")]
    pub description: Option<String>,
    #[serde(rename = "icy-genre")]
    pub genre: Option<String>,
    #[serde(rename = "icy-language-codes")]
    pub languages: Option<String>,
    #[serde(rename = "icy-country-code")]
    pub countrycode: Option<String>,
    #[serde(rename = "icy-country-subdivision-code")]
    pub country_subdivision_code: Option<String>,
    #[serde(rename = "icy-logo")]
    pub logo: Option<String>,
    #[serde(rename = "icy-geo-lat-long")]
    geo_lat_long: Option<String>,
}

impl MetaInfoFile {
    /// Decodes lat/long information contained in a stream information file
    pub fn get_lat_long(&self) -> Option<Result<LatLong, DecodeError>> {
        self.geo_lat_long.clone().map(|x| LatLong::try_from(x))
    }
}

pub fn extract_from_homepage(homepage: &str) -> Result<MetaInfoFile, Box<dyn Error>> {
    let stream_info_link = format!("{}/streaminfo.json", homepage);

    trace!(
        "extract_from_homepage({}) Download file '{}'",
        homepage,
        stream_info_link
    );
    let resp = get(&stream_info_link)?.text()?;
    let deserialized: MetaInfoFile = serde_json::from_str(&resp)?;
    Ok(deserialized)
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
