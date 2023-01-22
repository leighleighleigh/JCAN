#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Remove out and release folders
rm -rf "${SCRIPT_DIR}/out"
rm -rf "${SCRIPT_DIR}/release"

# Remove jcan-python/dist, build, **.egg-info folders
rm -rf "${SCRIPT_DIR}/jcan-python/dist"
rm -rf "${SCRIPT_DIR}/jcan-python/build"
rm -rf "${SCRIPT_DIR}/jcan-python/jcan.egg-info"


# Run cargo clean
cargo clean