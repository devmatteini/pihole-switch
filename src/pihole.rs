use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

use serde_json::Value as JsonValue;
use ureq::Response;

pub const PIHOLE_DEFAULT_HOST: &str = "pi.hole";

pub struct PiHoleConfig {
    pub api_token: String,
    pub api_url: String,
}

impl PiHoleConfig {
    pub fn new(api_token: String, api_url: String) -> PiHoleConfig {
        PiHoleConfig { api_token, api_url }
    }

    pub fn build_url(host: &str) -> String {
        format!("http://{}/admin/api.php", host)
    }
}

pub fn enable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_token);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, "enabled", PiHoleError::NotEnabled)
}

pub fn disable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?disable&auth={}", &config.api_url, &config.api_token);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, "disabled", PiHoleError::NotDisabled)
}

fn request(url: &str) -> Result<Response, PiHoleError> {
    let response = ureq::get(url).timeout_connect(7_000).call();

    if let Some(err) = response.synthetic_error() {
        return Err(PiHoleError::HttpError(err.body_text()));
    }

    Ok(response)
}

fn deserialize_response_json(response: Response) -> Result<JsonValue, PiHoleError> {
    response
        .into_json()
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
    HttpError(String),
    InvalidResponse,
    NotEnabled,
    NotDisabled,
}

impl fmt::Display for PiHoleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PiHoleError::BadRequestOrTokenNotValid => {
                f.write_str("Bad request or api token not valid")
            }
            PiHoleError::HttpError(inner) => f.write_str(&format!(
                "An error occurred during the request:\n  {}",
                inner
            )),
            PiHoleError::InvalidResponse => f.write_str("Pihole returned an invalid response"),
            PiHoleError::NotEnabled => f.write_str("Cannot enable pihole"),
            PiHoleError::NotDisabled => f.write_str("Cannot disable pihole"),
        }
    }
}

impl StdError for PiHoleError {}
