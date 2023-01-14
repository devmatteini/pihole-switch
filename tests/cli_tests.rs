mod support;

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    use assert_cmd::assert::OutputAssertExt;
    use assert_cmd::prelude::CommandCargoExt;

    use support::pihole_server::PiHoleServer;

    use super::*;

    #[test]
    fn successfully_enabled() {
        let api_token = "VALID_TOKEN".to_string();
        let url = PiHoleServer::new(api_token).start().unwrap();
        let host = get_host(&url);

        let mut cmd = Command::cargo_bin("phs").unwrap();

        let assert = cmd
            .env_clear()
            .args(["--host", &host])
            .args(["--token", "VALID_TOKEN"])
            .arg("enable")
            .assert();

        assert
            .success()
            .stdout(predicates::str::contains("enabled successfully"));
    }

    #[test]
    fn no_api_token_provided() {
        let mut cmd = Command::cargo_bin("phs").unwrap();

        let assert = cmd.env_clear().arg("enable").assert();

        assert
            .failure()
            .stderr(predicates::str::contains("No api token was provided"));
    }

    fn get_host(url: &str) -> String {
        let parts: Vec<&str> = url.split("http://").collect();

        parts[1].to_string()
    }
}
