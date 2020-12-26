use std::time::Duration;

use serde_json::Value as JsonValue;
use ureq::Response;

use crate::pihole::config::PiHoleConfig;
use crate::pihole::error::PiHoleError;

pub mod config;
pub mod error;

enum ExpectedStatus {
    Enabled,
    Disabled,
}

impl ExpectedStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ExpectedStatus::Enabled => "enabled",
            ExpectedStatus::Disabled => "disabled",
        }
    }
}

pub fn enable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_token);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, ExpectedStatus::Enabled, PiHoleError::NotEnabled)
}

pub fn disable(config: &PiHoleConfig, time: Option<Duration>) -> Result<(), PiHoleError> {
    let disable_time = time.unwrap_or_else(|| Duration::from_secs(0)).as_secs();
    let url = format!(
        "{}?disable={}&auth={}",
        &config.api_url, disable_time, &config.api_token
    );
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, ExpectedStatus::Disabled, PiHoleError::NotDisabled)
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
    expected_status: ExpectedStatus,
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
    expected: ExpectedStatus,
    actual: &JsonValue,
    error: PiHoleError,
) -> Result<(), PiHoleError> {
    if actual == &JsonValue::from(expected.as_str()) {
        Ok(())
    } else {
        Err(error)
    }
}
