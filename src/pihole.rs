use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

use serde_json::Value as JsonValue;

pub const PIHOLE_API_URL: &str = "http://pi.hole/admin/api.php";

pub struct PiHoleConfig {
    pub api_key: String,
    pub api_url: String,
}

pub fn enable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_key);
    let response = reqwest::blocking::get(&url).map_err(|_| PiHoleError::Unknown)?;

    let json = response.json::<JsonValue>().map_err(|_| PiHoleError::InvalidResponse)?;

    match json {
        JsonValue::Object(obj) => {
            match obj.get(&"status".to_string()) {
                Some(value) => validate_status(value),
                None => Err(PiHoleError::InvalidResponse)
            }
        }
        JsonValue::Array(_) => Err(PiHoleError::BadRequestOrTokenNotValid),
        _ => Err(PiHoleError::InvalidResponse)
    }
}

fn validate_status(value: &JsonValue) -> Result<(), PiHoleError> {
    if value == &JsonValue::from("enabled") {
        Ok(())
    } else {
        Err(PiHoleError::NotEnabled)
    }
}

#[derive(Debug, PartialEq)]
pub enum PiHoleError {
    BadRequestOrTokenNotValid,
    Unknown,
    InvalidResponse,
    NotEnabled,
}

impl fmt::Display for PiHoleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PiHoleError::BadRequestOrTokenNotValid => f.write_str("BadRequestOrTokenNotValid"),
            PiHoleError::Unknown => f.write_str("Unknown"),
            PiHoleError::InvalidResponse => f.write_str("InvalidResponse"),
            PiHoleError::NotEnabled => f.write_str("NotEnabled"),
        }
    }
}

impl StdError for PiHoleError {
    fn description(&self) -> &str {
        match *self {
            PiHoleError::BadRequestOrTokenNotValid => "No api token was provided",
            PiHoleError::Unknown => "Unknown error occurred during the request",
            PiHoleError::InvalidResponse => "Pihole returned an invalid response",
            PiHoleError::NotEnabled => "Cannot enable pihole",
        }
    }
}