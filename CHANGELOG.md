# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [v1.1.5] - 2025-03-25

This is a maintenance release that updates our dependencies.

### Updated dependencies

- bump `url` from 2.3.1 to 2.5.4
- bump `ureq` from 2.6.1 to 3.0.10
- bump `serde_json` from 0.10.0 to 3.2.0
- bump `predicates` from 2.1.5 to 3.1.3
- bump `console` from 0.15.4 to 0.15.11
- bump `clap` from 4.1.1 to 4.5.32
- bump `assert_cmd` from 2.0.8 to 2.0.16

## [v1.1.4] - 2023-01-14

This is a maintenance release that updates our dependencies.

### Development

- Rename git default branch `master` -> `main`

### Updated dependencies

- `clap` 3.2.22 -> 4.1.1
- `console` 0.15.1 -> 0.15.4
- `serde_json` 1.0.85 -> 1.0.91
- `ureq` 2.5.0 -> 2.6.1
- `serial_test` 0.9.0 -> 0.10.0
- `predicates` 2.1.1 -> 2.1.5
- `assert_cmd` 2.0.4 -> 2.0.8

## [v1.1.3] - 2022-09-17

### Security

`ansi_term` is [unmaintained](https://github.com/advisories/GHSA-74w3-p89x-ffgh), so we switched to the [`console`](https://crates.io/crates/console) crate to color text for the terminal.

### Updated dependencies

- `serde_json` 1.0.81 -> 1.0.85
- `ureq` 2.4.0 -> 2.5.0
- `clap` 3.1.18 -> 3.2.22

## [v1.1.2] - 2022-06-18

### Security

[CVE-2022-24713](https://github.com/advisories/GHSA-m5pq-gvj9-9vr8) - Updated `regex` crate to 1.5.6 ([#4](https://github.com/devmatteini/pihole-switch/pull/4))

## [v1.1.1] - 2022-06-11

### Development

- Update Rust edition to 2021
- Migration to `clap` as CLI argument parser instead of `structopt`

### Updated dependencies

- `serde_json` 1.0.59 -> 1.0.81
- `serial_test` 0.5.0 -> 0.7.0
- `url` 2.1.1 -> 2.2.2
- `assert_cmd` 1.0.1 -> 2.0.4
- `predicates` 1.0.5 -> 2.1.1
- `ureq` 1.5.1 -> 2.4.0

## [v1.1.0] - 2021-07-02

This release improves `pihole-switch` by polishing existing features.

### Added

- Automatic installation script from latest github release

### Changed

- Pihole api token must be specified using a global option `phs --token <token> <SUBCOMMAND>` instead of argument in
  enable/disable commands
- Improved CLI command handling ([#3](https://github.com/devmatteini/pihole-switch/pull/3))
- Restructured project folders

## [v1.0.0] - 2020-05-12

Initial release of `pihole-switch`.

### Added

- Enable pihole
- Disable pihole
- Disable pihole for custom time (in seconds)
- Specify pihole host (as an IpV4) when it's not the dns server of the device
