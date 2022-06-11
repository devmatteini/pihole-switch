#!/usr/bin/env bash

function usage() {
  echo "usage $(basename "$0") <version> <assets>..."
  echo
  echo "ARGS:"
  echo "    <version>    release version vX.Y.Z (e.g v1.3.0)"
  echo "    <assets>...  list of assets paths"
  exit 1
}

if [[ -z $1 || -z $2 ]]; then usage; fi
if [[ $1 != v* ]]; then
  echo "Version format must be: vX.Y.Z (e.g v1.3.0)"
  usage
fi
if ! command -v gh >/dev/null; then
  echo "Missing gh command"
  usage
fi
if [[ -z $GITHUB_REPOSITORY ]]; then
  echo "Missing GITHUB_REPOSITORY env variable. Format must be owner/repo"
  usage
fi

version="$1"
release_message=$(git log -1 --pretty=%B)

echo "version $version"

gh release create "$version" \
  --title "$version" \
  --notes "$release_message" \
  --repo "$GITHUB_REPOSITORY" \
  "${@:2}" # assets
