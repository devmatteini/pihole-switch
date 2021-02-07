# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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