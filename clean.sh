#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Remove out and release folders
rm -rf "${SCRIPT_DIR}/out"
rm -rf "${SCRIPT_DIR}/release"

# Remove jorzacan-python/dist, build, **.egg-info folders
rm -rf "${SCRIPT_DIR}/jorzacan-python/dist"
rm -rf "${SCRIPT_DIR}/jorzacan-python/build"
rm -rf "${SCRIPT_DIR}/jorzacan-python/jorzacan.egg-info"


# Run cargo clean
cargo clean