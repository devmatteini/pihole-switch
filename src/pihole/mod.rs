use std::time::Duration;

use serde_json::Value as JsonValue;
use ureq::Response;

use crate::pihole::config::PiHoleConfig;
use crate::pihole::error::PiHoleError;

pub mod config;
pub mod error;

pub fn enable(config: &PiHoleConfig) -> Result<(), PiHoleError> {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_token);
    let response = request(&url)?;

    let json = deserialize_response_json(response)?;

    process_response(json, "enabled", PiHoleError::NotEnabled)
}

pub fn disable(config: &PiHoleConfig, time: Option<Duration>) -> Result<(), PiHoleError> {
    let disable_time = time.unwrap_or_else(|| Duration::from_secs(0)).as_secs();
    let url = format!(
        "{}?disable={}&auth={}",
        &config.api_url, disable_time, &config.api_token
    );
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
