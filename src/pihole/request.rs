use serde_json::Value as JsonValue;

use crate::pihole::error::PiHoleError;

pub fn get(url: &str) -> Result<JsonValue, PiHoleError> {
    let response = ureq::get(url).timeout_connect(7_000).call();

    if let Some(err) = response.synthetic_error() {
        return Err(PiHoleError::HttpError(err.body_text()));
    }

    response
        .into_json()
        .map_err(|_| PiHoleError::InvalidResponse)
}
