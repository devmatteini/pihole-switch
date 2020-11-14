use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pihole-switch",
    about = "A command line tool to enable/disable your PiHole"
)]
pub struct Cli {
    /// Override default pihole host
    ///
    /// You can pass an IPv4/hostname to override the default host (`pi.hole`) in order to make
    /// pihole-switch work if it's not set as the device dns server.
    #[structopt(short = "H", long = "host")]
    pub host: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Enable your PiHole
    ///
    /// The api token is required.
    /// You can either pass the `[token]` argument or set an environment variable `PIHOLE_TOKEN`
    ///
    /// You can find the PiHole api token at http://pi.hole/admin/settings.php?tab=api,
    /// then under the section `Query log`, click on the button `Show API token` and confirm.
    #[structopt(alias = "e")]
    Enable {
        #[structopt(help = "PiHole api token to make authorized requests")]
        token: Option<String>,
    },
    #[structopt(alias = "d")]
    /// Disable your PiHole
    ///
    /// The api token is required.
    /// You can either pass the `[token]` argument or set an environment variable `PIHOLE_TOKEN`
    ///
    /// You can find the PiHole api token at http://pi.hole/admin/settings.php?tab=api,
    /// then under the section `Query log`, click on the button `Show API token` and confirm.
    Disable {
        #[structopt(help = "PiHole api token to make authorized requests")]
        token: Option<String>,

        /// Disable pihole for custom seconds
        ///
        /// Default is indefinitely.
        #[structopt(short = "t", long = "time")]
        time: Option<u64>,
    },
}
