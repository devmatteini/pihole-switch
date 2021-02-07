# Pihole Switch

![CI](https://github.com/devmatteini/pihole-switch/workflows/CI/badge.svg)

A command line tool to enable/disable your [pihole](https://pi-hole.net/)

![](./assets/phs_demo.gif)

Sometimes you just need to disable your pihole for some website to work.
_**PiholeSwitch** makes this operation much easier_.

Just open a terminal, and you can disable pihole in a moment and for as long as you want.

## Table of contents

- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Installation

#### Recommended

```bash
$ curl -s https://raw.githubusercontent.com/devmatteini/pihole-switch/master/install.sh | bash
```

Download the [latest release](https://github.com/devmatteini/pihole-switch/releases/latest)
and unzip the `phs.zip` file.

After the archive is unzipped, move the `phs` binary in a folder included in your `$PATH` (it's commonly used `/usr/local/bin`).

#### From source

```bash
git clone https://github.com/devmatteini/pihole-switch && cd pihole-switch
cargo build --release
./target/release/phs
```

## Usage

Export an environment variable `PIHOLE_TOKEN=<pihole_api_token>`.

You can find the api token here: http://pi.hole/admin/settings.php?tab=api, then under the section `Query log`, click on
the button `Show API token` and confirm.

### Commands

Enable pihole

```bash
$ phs enable
```

Disable pihole (indefinitely)

```bash
$ phs disable
```

Disable pihole for 20 seconds

```bash
$ phs disable --time 20
```

If your device doesn't have pihole set as dns

```bash
$ phs --host 192.168.1.50 disable
```

(where `192.168.1.50` is the pihole ip address)

Enable/disable by passing the api token as an option, instead of using the `PIHOLE_TOKEN` environment variable

```bash
$ phs --token <token> <SUBCOMMAND>
```

_**Note: the api token passed as an option will always have priority on the `PIHOLE_TOKEN` environment variable**_

## License

`pihole-switch` is made available under the terms of the MIT License.

See the [MIT License](LICENSE) file for license details.