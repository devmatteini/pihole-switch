use structopt::StructOpt;

use pihole_switch::pihole;
use pihole_switch::pihole::PiHoleConfig;
use pihole_switch::resolve_api_token::resolve_api_token;

use crate::cli::io::{print_error, print_pihole_error, print_success};
use crate::cli::root_command::Cli;

mod cli;

fn main() {
    match Cli::from_args() {
        Cli::Enable { token } => handle_enable(token),
        Cli::Disable { token } => handle_disable(token),
    }
}

fn handle_enable(token: Option<String>) {
    match resolve_api_token(token) {
        Ok(token) => {
            let config = PiHoleConfig::new(token);

            let res = pihole::enable(&config);

            match res {
                Ok(_) => print_success("PiHole enabled successfully!"),
                Err(err) => print_pihole_error(err),
            }
        }
        Err(err) => print_error(&format!(
            "{}\nUse `pihole-switch enable [token]` or set PIHOLE_TOKEN environment variable",
            err
        )),
    }
}

fn handle_disable(token: Option<String>) {
    match resolve_api_token(token) {
        Ok(token) => {
            let config = PiHoleConfig::new(token);

            let res = pihole::disable(&config);

            match res {
                Ok(_) => print_success("PiHole disabled successfully!"),
                Err(err) => print_pihole_error(err),
            }
        }
        Err(err) => print_error(&format!(
            "{}\nUse `pihole-switch disable [token]` or set PIHOLE_TOKEN environment variable",
            err
        )),
    }
}
