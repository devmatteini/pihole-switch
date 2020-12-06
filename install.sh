#!/bin/bash

set -euo pipefail

readonly RESET="\e[0m"
readonly GREEN="\e[1;32m"
readonly YELLOW="\e[1;33m"
readonly RED="\e[1;31m"
readonly PURPLE="\e[1;35m"

readonly LATEST_RELEASE_URL="https://github.com/devmatteini/pihole-switch/releases/latest/download/phs.zip"

fetch() {
  curl -sL "$LATEST_RELEASE_URL" --output "$1"
}

install() {
  local asset_path
  asset_path=$(mktemp)

  echo -e "${PURPLE}Downloading latest release of pihole-switch...${RESET}"
  fetch "$asset_path"

  echo -e "${YELLOW}Escalated permissions are required to install to /usr/local/bin${RESET}"
  if sudo unzip "$asset_path" -d /usr/local/bin >/dev/null; then
    echo -e "${GREEN}PiholeSwitch installed${RESET}"
  else
    echo -e "${RED}PiholeSwitch not installed${RESET}"
  fi

  rm "$asset_path"
}

install
