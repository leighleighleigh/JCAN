#!/usr/bin/env bash

# Same as build.sh, but uses 'cross-rs' first

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Activate virtual environment if needed
if [[ -z "${VIRTUAL_ENV}" ]];
then
    echo "Activating .venv"
    source ${SCRIPT_DIR}/.venv/bin/activate
fi

# Installs build tools just incase they arent
#cargo install cross --git https://github.com/cross-rs/cross
#sudo apt install podman

# Very important to clean, incase old crates for x86 are present
cargo clean

export CROSS_CONTAINER_ENGINE=docker

CROSSTARGET="aarch64-unknown-linux-gnu"

cross build --package jorzacan --target $CROSSTARGET --release
cross build --package scripts_postbuild --target $CROSSTARGET --release

# python build uses a special pyo3 image
export CARGO=cross
export CARGO_BUILD_TARGET=${CROSSTARGET}
export CROSS_CONFIG=${SCRIPT_DIR}/jorzacan-python/Cross.toml

cross build --package jorzacan_python --target $CROSSTARGET --release

# Run setuptools-rust on jorzacan_python
cd jorzacan-python
rm -rf ./dist
rm -rf ./build

python setup.py bdist_wheel --plat-name manylinux2014_aarch64 --py-limited-api=cp38
cd ..

# Copy wheels to out folder
mkdir -p out/wheels
cp -r jorzacan-python/dist/* out/wheels/
