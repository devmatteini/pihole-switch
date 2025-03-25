use serde_json::Value as JsonValue;

use crate::pihole::error::PiHoleError;

pub fn get(url: &str) -> Result<JsonValue, PiHoleError> {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(7)))
        .build();
    let agent: ureq::Agent = config.into();

    let mut response = agent.get(url).call().map_err(pihole_error)?;
    deserialize(response.body_mut())
}

fn deserialize(x: &mut ureq::Body) -> Result<JsonValue, PiHoleError> {
    x.read_json().map_err(|_| PiHoleError::InvalidResponse)
}

fn pihole_error(error: ureq::Error) -> PiHoleError {
    PiHoleError::HttpError(error.to_string())
}
