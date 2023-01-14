use std::net::{AddrParseError, Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use clap::Parser;

fn try_parse_host(src: &str) -> Result<String, AddrParseError> {
    if src.contains(':') {
        return SocketAddrV4::from_str(src).map(|x| x.to_string());
    }

    Ipv4Addr::from_str(src).map(|x| x.to_string())
}

#[derive(Debug, Parser)]
#[command(
    name = "pihole-switch",
    version,
    about = "A command line tool to enable/disable your PiHole"
)]
pub struct Cli {
    /// Override default pihole host
    ///
    /// You can pass an IPv4 to override the default host (`pi.hole`) in order to make
    /// pihole-switch work if it's not set as the device dns server.
    #[arg(short = 'H', long = "host", value_parser = try_parse_host)]
    pub host: Option<String>,

    /// PiHole's api token
    ///
    /// You can either use this option or set an environment variable `PIHOLE_TOKEN`.
    ///
    /// You can find the PiHole api token at http://pi.hole/admin/settings.php?tab=api,
    /// then under the section `Query log`, click on the button `Show API token` and confirm.
    #[arg(short = 'T', long = "token")]
    pub token: Option<String>,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Enable your PiHole
    ///
    /// The api token is required.
    /// Use `phs --token <api_token> enable` or set set an environment variable `PIHOLE_TOKEN`
    #[command(alias("e"))]
    Enable,

    #[command(alias("d"))]
    /// Disable your PiHole
    ///
    /// The api token is required.
    /// Use `phs --token <api_token> disable` or set set an environment variable `PIHOLE_TOKEN`
    Disable {
        /// Disable pihole for custom seconds
        ///
        /// Default is indefinitely.
        #[arg(short = 't', long = "time")]
        time: Option<u64>,
    },
}
