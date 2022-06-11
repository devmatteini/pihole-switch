use clap::Parser;

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
    let args: Cli = Cli::parse();

    let host = args.host.unwrap_or_else(|| PIHOLE_DEFAULT_HOST.to_string());

    let token = match resolve_api_token(args.token) {
        Ok(value) => value,
        Err(err) => {
            print_error(&format!(
                "{}\n{}",
                err,
                "Use `phs --token <token> <SUBCOMMAND>` or set `PIHOLE_TOKEN` environment variable"
            ));
            return ExitCode::Error;
        }
    };

    match args.cmd {
        Command::Enable => handle_enable(host, token),
        Command::Disable { time } => handle_disable(host, token, time),
    }
}

fn main() {
    let exit_code = run_app();
    std::process::exit(exit_code as i32);
}

struct CommandMessages {
    pub ok: String,
}

fn handle_enable(host: String, token: String) -> ExitCode {
    handle_command(
        token,
        host,
        pihole::enable,
        CommandMessages {
            ok: "PiHole enabled successfully!".to_string(),
        },
    )
}

fn handle_disable(host: String, token: String, time: Option<u64>) -> ExitCode {
    handle_command(
        token,
        host,
        |conf: &PiHoleConfig| {
            let disable_time = PiHoleDisableTime::from_secs(time);
            pihole::disable(conf, disable_time)
        },
        CommandMessages {
            ok: "PiHole disabled successfully!".to_string(),
        },
    )
}

fn handle_command<F>(
    token: String,
    host: String,
    cmd_func: F,
    messages: CommandMessages,
) -> ExitCode
where
    F: FnOnce(&PiHoleConfig) -> PiholeResult,
{
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

fn build_pihole_config(token: String, host: &str) -> PiHoleConfig {
    let api_url = PiHoleConfig::build_url(host);
    PiHoleConfig::new(token, api_url)
}
