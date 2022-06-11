use serde_json::Value as JsonValue;

use crate::pihole::error::PiHoleError;

pub fn get(url: &str) -> Result<JsonValue, PiHoleError> {
    ureq::get(url)
        .timeout(std::time::Duration::from_secs(7))
        .call()
        .map_err(pihole_error)
        .and_then(deserialize)
}

fn deserialize(x: ureq::Response) -> Result<JsonValue, PiHoleError> {
    x.into_json().map_err(|_| PiHoleError::InvalidResponse)
}

fn pihole_error(error: ureq::Error) -> PiHoleError {
    match error {
        ureq::Error::Transport(e) => {
            let message = e.message().unwrap_or("Unknown transport error");
            let kind = e.kind();
            PiHoleError::HttpError(format!("{} - {}", kind, message))
        }
        other => PiHoleError::HttpError(other.to_string())
    }
}
