use serde_json::Value as JsonValue;

use crate::pihole::config::PiHoleConfig;
use crate::pihole::disable_time::PiHoleDisableTime;
use crate::pihole::error::PiHoleError;

pub mod config;
pub mod disable_time;
pub mod error;
mod request;

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

pub type PiholeResult = Result<(), PiHoleError>;

pub fn enable(config: &PiHoleConfig) -> PiholeResult {
    let url = format!("{}?enable&auth={}", &config.api_url, &config.api_token);
    let response = request::get(&url)?;

    process_response(response, ExpectedStatus::Enabled, PiHoleError::NotEnabled)
}

pub fn disable(config: &PiHoleConfig, time: PiHoleDisableTime) -> PiholeResult {
    let url = format!(
        "{}?disable={}&auth={}",
        &config.api_url,
        time.as_secs(),
        &config.api_token
    );
    let response = request::get(&url)?;

    process_response(response, ExpectedStatus::Disabled, PiHoleError::NotDisabled)
}

fn process_response(
    json: JsonValue,
    expected_status: ExpectedStatus,
    status_error: PiHoleError,
) -> PiholeResult {
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
) -> PiholeResult {
    if actual == &JsonValue::from(expected.as_str()) {
        Ok(())
    } else {
        Err(error)
    }
}
