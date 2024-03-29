use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

pub const PIHOLE_TOKEN_ENV: &str = "PIHOLE_TOKEN";

pub fn resolve_api_token(api_token: Option<String>) -> Result<String, TokenResolverError> {
    match api_token {
        Some(value) => Ok(value),
        None => match std::env::var(PIHOLE_TOKEN_ENV) {
            Ok(value) => Ok(value),
            Err(_) => Err(TokenResolverError::NoToken),
        },
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenResolverError {
    NoToken,
}

impl fmt::Display for TokenResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TokenResolverError::NoToken => f.write_str("No api token was provided"),
        }
    }
}

impl StdError for TokenResolverError {}

#[cfg(test)]
mod tests {
    use std::env;

    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn only_parameter() {
        set_api_token_env(None);
        let api_token = Some(String::from("ANY_VALUE"));

        let actual = resolve_api_token(api_token);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE")));
    }

    #[test]
    #[serial]
    fn only_env_variable() {
        set_api_token_env(Some("ANY_VALUE"));

        let actual = resolve_api_token(None);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE")));
    }

    #[test]
    #[serial]
    fn parameter_has_priority_on_env_var() {
        set_api_token_env(Some("ANY_VALUE"));
        let api_token = Some(String::from("ANY_VALUE_AS_PARAM"));

        let actual = resolve_api_token(api_token);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE_AS_PARAM")));
    }

    #[test]
    #[serial]
    fn no_api_token() {
        set_api_token_env(None);
        let actual = resolve_api_token(None);

        assert_eq!(actual.err(), Some(TokenResolverError::NoToken));
    }

    fn set_api_token_env(api_token: Option<&str>) {
        match api_token {
            Some(value) => env::set_var(PIHOLE_TOKEN_ENV, value),
            None => {
                if env::var(PIHOLE_TOKEN_ENV).is_ok() {
                    env::remove_var(PIHOLE_TOKEN_ENV);
                }
            }
        }
    }
}
