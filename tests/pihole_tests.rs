mod support;

#[cfg(test)]
mod pihole_tests {
    use pihole_switch::pihole;
    use pihole_switch::pihole::{PiHoleConfig, PiHoleError};
    use support::pihole_server::PiHoleServer;

    use super::*;

    #[test]
    fn successfully_enabled() {
        let api_key = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_key).start().unwrap();

        let config = PiHoleConfig {
            api_key: "VALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Ok(()))
    }

    #[test]
    fn enable_with_invalid_token() {
        let api_key = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_key).start().unwrap();

        let config = PiHoleConfig {
            api_key: "INVALID_TOKEN".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    #[test]
    fn enable_with_empty_token() {
        let api_key = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_key).start().unwrap();

        let config = PiHoleConfig {
            api_key: "".to_string(),
            api_url: url,
        };

        let response = pihole::enable(&config);

        assert_eq!(response, Err(PiHoleError::BadRequestOrTokenNotValid))
    }

    #[test]
    fn no_server_found() {
        let config = PiHoleConfig {
            api_key: "VALID_TOKEN".to_string(),
            api_url: "http://localhost:0".to_string(),
        };

        let response = pihole::enable(&config);

        assert_eq!(response.err(), Some(PiHoleError::Unknown));
    }
}