use std::error::Error;

use reqwest::blocking::get;

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaInfoFile {
    #[serde(rename = "icy-index-metadata")]
    pub index_metadata: u8,
    #[serde(rename = "icy-version")]
    pub version: u8,
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
    #[serde(rename = "icy-country-subdivison-code")]
    pub country_subdivision_code: Option<String>,
    #[serde(rename = "icy-logo")]
    pub logo: Option<String>,
}

pub fn extract_from_homepage(homepage: &str) -> Result<MetaInfoFile, Box<dyn Error>> {
    let stream_info_link = format!("{}/streaminfo.json", homepage);

    trace!("extract_from_homepage({}) Download file '{}'", homepage, stream_info_link);
    let resp = get(&stream_info_link)?.text()?;
    let deserialized: MetaInfoFile = serde_json::from_str(&resp)?;
    
    Ok(deserialized)
}
