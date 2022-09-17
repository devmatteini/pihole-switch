use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq)]
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

impl Error for PiHoleError {}
