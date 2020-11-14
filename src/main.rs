use structopt::StructOpt;

use pihole_switch::pihole;
use pihole_switch::pihole::PiHoleConfig;
use pihole_switch::resolve_api_token::resolve_api_token;

use crate::cli::io::{print_error, print_pihole_error, print_success};
use crate::cli::root_command::{Cli, Command};

mod cli;

enum ExitCode {
    Ok = 0,
    Error = 1,
}

fn main() {
    let args: Cli = Cli::from_args();

    let host = args.host;

    let exit_code = match args.cmd {
        Command::Enable { token } => handle_enable(token, host),
        Command::Disable { token } => handle_disable(token, host),
    };

    std::process::exit(exit_code as i32);
}

fn handle_enable(token: Option<String>, host: Option<String>) -> ExitCode {
    match resolve_api_token(token) {
        Ok(token) => {
            let config = build_pihole_config(token, host);

            let res = pihole::enable(&config);

            match res {
                Ok(_) => print_success("PiHole enabled successfully!"),
                Err(err) => {
                    print_pihole_error(err);
                    return ExitCode::Error;
                }
            }
        }
        Err(err) => {
            print_error(&format!(
                "{}\nUse `phs enable [token]` or set PIHOLE_TOKEN environment variable",
                err
            ));

            return ExitCode::Error;
        }
    }

    ExitCode::Ok
}

fn handle_disable(token: Option<String>, host: Option<String>) -> ExitCode {
    match resolve_api_token(token) {
        Ok(token) => {
            let config = build_pihole_config(token, host);

            let res = pihole::disable(&config, None);

            match res {
                Ok(_) => print_success("PiHole disabled successfully!"),
                Err(err) => {
                    print_pihole_error(err);
                    return ExitCode::Error;
                }
            }
        }
        Err(err) => {
            print_error(&format!(
                "{}\nUse `phs disable [token]` or set PIHOLE_TOKEN environment variable",
                err
            ));

            return ExitCode::Error;
        }
    }

    ExitCode::Ok
}

fn build_pihole_config(token: String, host: Option<String>) -> PiHoleConfig {
    let host = host.unwrap_or_else(|| pihole::PIHOLE_DEFAULT_HOST.to_string());
    let api_url = PiHoleConfig::build_url(&host);

    PiHoleConfig::new(token, api_url)
}
