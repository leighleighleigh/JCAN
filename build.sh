#!/usr/bin/env bash


SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Activate virtual environment if needed
if [[ -z "${VIRTUAL_ENV}" ]];
then
    echo "Activating .venv"
    source ${SCRIPT_DIR}/.venv/bin/activate
fi

# Builds the library, and generates C++ headers
cargo build --release --package jorzacan
# Moves build artifacts to out/<profile>/<target>/jorzacan/
cargo build --release --package scripts_postbuild

# Unsure if this is needed, but it might be, so I'm running it anyway
cargo build --release --package jorzacan_python
# Packages the libjorzacan_python.so file from the build above, into a python wheel
cd jorzacan-python
maturin build
cd ..
# Copy the wheel to out/wheels/
cp -r target/wheels out/
