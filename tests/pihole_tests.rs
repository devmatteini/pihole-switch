extern crate core;

mod support;

#[cfg(test)]
mod pihole_tests {
    use std::time::Duration;

    use predicates::Predicate;

    use pihole_switch::pihole;
    use pihole_switch::pihole::config::PiHoleConfig;
    use pihole_switch::pihole::disable_time::PiHoleDisableTime;
    use pihole_switch::pihole::error::PiHoleError;
    use support::pihole_server::PiHoleServer;

    use super::*;

    #[test]
    fn successfully_enabled() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "VALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Ok(()))
    }

    #[test]
    fn enable_with_invalid_token() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "INVALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    #[test]
    fn enable_with_empty_token() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    #[test]
    fn no_server_found() {
        let config = PiHoleConfig {
            api_token: "VALID_TOKEN".to_string(),
            api_url: "http://localhost:0".to_string(),
        };

        let response = pihole::enable(&config);

        let error = assert_error(response.err());
        assert_http_error(predicates::str::contains("Connection refused"), error);
    }

    #[test]
    fn successfully_disabled() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "VALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::disable(&config, PiHoleDisableTime::none());

        assert_eq!(response, Ok(()))
    }

    #[test]
    fn successfully_disabled_with_time() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "VALID_TOKEN".to_string(),
            api_url: url,
        };
        let two_secs = PiHoleDisableTime::from_duration(Duration::from_secs(2));

        let response = pihole::disable(&config, two_secs);

        assert_eq!(response, Ok(()))
    }

    #[test]
    fn disable_with_invalid_token() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "INVALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::disable(&config, PiHoleDisableTime::none());

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    #[test]
    fn disable_with_empty_token() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();

        let config = PiHoleConfig {
            api_token: "".to_string(),
            api_url: url,
        };

        let response = pihole::disable(&config, PiHoleDisableTime::none());

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    fn assert_error(e: Option<PiHoleError>) -> PiHoleError {
        match e {
            Some(x) => x,
            None => panic!("No PiHole error"),
        }
    }

    fn assert_http_error(predicate: predicates::str::ContainsPredicate, error: PiHoleError) {
        match error {
            PiHoleError::HttpError(e) => assert!(predicate.eval(&e)),
            x => panic!("PiHoleError is not an Http error but {:?}", x),
        }
    }
}
