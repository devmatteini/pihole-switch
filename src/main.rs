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

fn run_app() -> ExitCode {
    let args: Cli = Cli::from_args();

    let host = args.host.unwrap_or_else(|| PIHOLE_DEFAULT_HOST.to_string());

    match args.cmd {
        Command::Enable { token } => handle_enable(host, token),
        Command::Disable { token, time } => handle_disable(host, token, time),
    }
}

fn main() {
    let exit_code = run_app();
    std::process::exit(exit_code as i32);
}

struct CommandMessages {
    pub ok: String,
    pub api_token_error: String,
}

fn handle_enable(host: String, token: Option<String>) -> ExitCode {
    handle_command(
        token,
        host,
        |conf: &PiHoleConfig| pihole::enable(conf),
        CommandMessages {
            ok: "PiHole enabled successfully!".to_string(),
            api_token_error: "Use `phs enable [token]` or set PIHOLE_TOKEN environment variable"
                .to_string(),
        },
    )
}

fn handle_disable(host: String, token: Option<String>, time: Option<u64>) -> ExitCode {
    handle_command(
        token,
        host,
        |conf: &PiHoleConfig| {
            let disable_time = PiHoleDisableTime::from_secs(time);
            pihole::disable(conf, disable_time)
        },
        CommandMessages {
            ok: "PiHole disabled successfully!".to_string(),
            api_token_error: "Use `phs disable [token]` or set PIHOLE_TOKEN environment variable"
                .to_string(),
        },
    )
}

fn handle_command<F>(
    token: Option<String>,
    host: String,
    cmd_func: F,
    messages: CommandMessages,
) -> ExitCode
where
    F: FnOnce(&PiHoleConfig) -> PiholeResult,
{
    match resolve_api_token(token) {
        Ok(token) => {
            let config = build_pihole_config(token, &host);
            match cmd_func(&config) {
                Ok(_) => {
                    print_success(&messages.ok);
                    ExitCode::Ok
                }
                Err(err) => {
                    print_pihole_error(err);
                    ExitCode::Error
                }
            }
        }
        Err(err) => {
            print_error(&format!("{}\n{}", err, &messages.api_token_error));
            ExitCode::Error
        }
    }
}

fn build_pihole_config(token: String, host: &str) -> PiHoleConfig {
    let api_url = PiHoleConfig::build_url(host);
    PiHoleConfig::new(token, api_url)
}
