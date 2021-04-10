use crate::DecodeError;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

/// Represents a geo-location with latitude and longitude. It can be
/// constructed from a String.
/// 
/// # Example
/// ```rust
/// use std::convert::TryFrom;
/// use av_stream_info_rust::LatLong;
/// 
/// let lat_long_str = String::from("10.1,-3.1");
/// let lat_long = LatLong::try_from(lat_long_str).unwrap();
/// println!("{},{}", lat_long.lat, lat_long.long);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LatLong {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub long: f64,
}

impl TryFrom<String> for LatLong {
    type Error = DecodeError;

    fn try_from(
        lat_long_str: String,
    ) -> std::result::Result<Self, <Self as TryFrom<String>>::Error> {
        let mut iter = lat_long_str.splitn(2, ",");
        Ok(LatLong {
            lat: iter
                .next()
                .ok_or(DecodeError::LatMissing)?
                .parse()
                .or(Err(DecodeError::NumberParseError))?,
            long: iter
                .next()
                .ok_or(DecodeError::LongMissing)?
                .parse()
                .or(Err(DecodeError::NumberParseError))?,
        })
    }
}
