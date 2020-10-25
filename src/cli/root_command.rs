use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pihole-switch",
    about = "A command line tool to enable/disable your PiHole",
    no_version
)]
pub enum Cli {
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
    },
}
