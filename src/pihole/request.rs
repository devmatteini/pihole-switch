use serde_json::Value as JsonValue;

use crate::pihole::error::PiHoleError;

pub fn get(url: &str) -> Result<JsonValue, PiHoleError> {
    ureq::get(url)
        .timeout(std::time::Duration::from_secs(7)).call()
        .map_err(|e| PiHoleError::HttpError(e.to_string()))
        .and_then(deserialize)
}

fn deserialize(x: ureq::Response) -> Result<JsonValue, PiHoleError> {
    x.into_json().map_err(|_| PiHoleError::InvalidResponse)
}
