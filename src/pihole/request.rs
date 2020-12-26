use serde_json::Value;
use ureq::Response;

use crate::pihole::error::PiHoleError;

pub fn get(url: &str) -> Result<Response, PiHoleError> {
    let response = ureq::get(url).timeout_connect(7_000).call();

    if let Some(err) = response.synthetic_error() {
        return Err(PiHoleError::HttpError(err.body_text()));
    }

    Ok(response)
}

pub fn deserialize_response_json(response: Response) -> Result<Value, PiHoleError> {
    response
        .into_json()
        .map_err(|_| PiHoleError::InvalidResponse)
}
