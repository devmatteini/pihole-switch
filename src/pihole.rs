use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

use reqwest::blocking::Response;
use serde_json::Value as JsonValue;

pub const PIHOLE_API_URL: &str = "http://pi.hole/admin/api.php";

pub struct PiHoleConfig {
    pub api_key: String,
    pub api_url: String,
}

impl PiHoleConfig {
    pub fn new(api_key: String) -> PiHoleConfig {
        PiHoleConfig {
            api_key,
            api_url: PIHOLE_API_URL.to_string(),
        }
    }
}

pub fn enable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_key);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, "enabled", PiHoleError::NotEnabled)
}

pub fn disable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?disable&auth={}", &config.api_url, &config.api_key);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, "disabled", PiHoleError::NotDisabled)
}

fn request(url: &String) -> Result<Response, PiHoleError> {
    reqwest::blocking::get(url).map_err(|_| PiHoleError::Unknown)
}

fn deserialize_response_json(response: Response) -> Result<JsonValue, PiHoleError> {
    response
        .json::<JsonValue>()
        .map_err(|_| PiHoleError::InvalidResponse)
}

fn process_response(
    json: JsonValue,
    expected_status: &str,
    status_error: PiHoleError,
) -> Result<(), PiHoleError> {
    match json {
        JsonValue::Object(obj) => match obj.get(&"status".to_string()) {
            Some(actual) => validate_status(expected_status, actual, status_error),
            None => Err(PiHoleError::InvalidResponse),
        },
        JsonValue::Array(_) => Err(PiHoleError::BadRequestOrTokenNotValid),
        _ => Err(PiHoleError::InvalidResponse),
    }
}

fn validate_status(
    expected: &str,
    actual: &JsonValue,
    error: PiHoleError,
) -> Result<(), PiHoleError> {
    if actual == &JsonValue::from(expected) {
        Ok(())
    } else {
        Err(error)
    }
}

#[derive(Debug, PartialEq)]
pub enum PiHoleError {
    BadRequestOrTokenNotValid,
    Unknown,
    InvalidResponse,
    NotEnabled,
    NotDisabled,
}

impl fmt::Display for PiHoleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PiHoleError::BadRequestOrTokenNotValid => {
                f.write_str("Bad request or api key not valid")
            }
            PiHoleError::Unknown => f.write_str("Unknown error occurred during the request"),
            PiHoleError::InvalidResponse => f.write_str("Pihole returned an invalid response"),
            PiHoleError::NotEnabled => f.write_str("Cannot enable pihole"),
            PiHoleError::NotDisabled => f.write_str("Cannot disable pihole"),
        }
    }
}

impl StdError for PiHoleError {}
