use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;

pub fn resolve_api_token(api_token: Option<String>) -> Result<String, TokenResolverError> {
    let env = std::env::var("PIHOLE_TOKEN");

    match env {
        Ok(value) => Ok(value),
        Err(_) => match api_token {
            Some(value) => Ok(value),
            None => Err(TokenResolverError::NoToken),
        },
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenResolverError {
    NoToken,
}

impl fmt::Display for TokenResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TokenResolverError::NoToken => f.write_str("NoToken"),
        }
    }
}

impl StdError for TokenResolverError {
    fn description(&self) -> &str {
        match *self {
            TokenResolverError::NoToken => "No api token was provided",
        }
    }
}

#[cfg(test)]
mod resolve_api_token_tests {
    use std::env;

    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn with_env_var() {
        set_api_token_env(Some("ANY_VALUE"));

        let actual = resolve_api_token(None);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE")));
    }

    #[test]
    #[serial]
    fn with_env_var_and_parameter() {
        set_api_token_env(Some("ANY_VALUE"));
        let api_token = Some(String::from("ANY_VALUE_AS_PARAM"));

        let actual = resolve_api_token(api_token);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE")));
    }

    #[test]
    #[serial]
    fn without_env_var() {
        set_api_token_env(None);
        let api_token = Some(String::from("ANY_VALUE"));

        let actual = resolve_api_token(api_token);

        assert_eq!(actual.ok(), Some(String::from("ANY_VALUE")));
    }

    #[test]
    #[serial]
    fn no_api_token() {
        set_api_token_env(None);
        let actual = resolve_api_token(None);

        assert_eq!(actual.err(), Some(TokenResolverError::NoToken));
    }

    fn set_api_token_env(api_token: Option<&str>) {
        let env_key = "PIHOLE_TOKEN";

        match api_token {
            Some(value) => env::set_var(env_key, value),
            None => {
                if env::var(env_key).is_ok() {
                    env::remove_var(env_key);
                }
            }
        }
    }
}
