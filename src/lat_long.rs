use crate::DecodeError;
use std::convert::TryFrom;

#[derive(Debug, Serialize, Clone)]
pub struct LatLong {
    pub lat: f64,
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
