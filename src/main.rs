use structopt::StructOpt;

use pihole_switch::pihole;
use pihole_switch::pihole::config::{PiHoleConfig, PIHOLE_DEFAULT_HOST};
use pihole_switch::pihole::disable_time::PiHoleDisableTime;
use pihole_switch::pihole::PiholeResult;
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
        Command::Enable { token } => handle_command(
            token,
            host,
            &|conf: &PiHoleConfig| pihole::enable(conf),
            &"enable",
        ),
        Command::Disable { token, time } => handle_command(
            token,
            host,
            &|conf: &PiHoleConfig| {
                let disable_time = PiHoleDisableTime::from_secs(time);
                pihole::disable(conf, disable_time)
            },
            &"disable",
        ),
    };

    std::process::exit(exit_code as i32);
}

fn handle_command(
    token: Option<String>,
    host: Option<String>,
    cmd_func: &dyn Fn(&PiHoleConfig) -> PiholeResult,
    cmd_name: &str,
) -> ExitCode {
    match resolve_api_token(token) {
        Ok(token) => {
            let config = build_pihole_config(token, host);
            match cmd_func(&config) {
                Ok(_) => {
                    print_success(&format!("PiHole {}d successfully!", cmd_name));
                    ExitCode::Ok
                }
                Err(err) => {
                    print_pihole_error(err);
                    ExitCode::Error
                }
            }
        }
        Err(err) => {
            print_error(&format!(
                "{}\nUse `phs {} [token]` or set PIHOLE_TOKEN environment variable",
                err, cmd_name
            ));

            ExitCode::Error
        }
    }
}

fn build_pihole_config(token: String, host: Option<String>) -> PiHoleConfig {
    let host = host.unwrap_or_else(|| PIHOLE_DEFAULT_HOST.to_string());
    let api_url = PiHoleConfig::build_url(&host);

    PiHoleConfig::new(token, api_url)
}
