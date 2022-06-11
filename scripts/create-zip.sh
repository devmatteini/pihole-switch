#!/usr/bin/env bash

CURRENT_DIR=$(pwd)

strip ./target/release/phs
(cd ./target/release && zip "${CURRENT_DIR}/phs.zip" ./phs)
