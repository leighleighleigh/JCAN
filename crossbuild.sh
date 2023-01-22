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

# Check for 'cross' command, else prompt install
if ! command -v cross &> /dev/null
then
    echo "cross could not be found, please install with:"
    echo "cargo install cross --git https://github.com/cross-rs/cross"
    exit 1
fi

# Check for 'podman' command, else prompt install
if ! command -v podman &> /dev/null
then
    echo "podman could not be found, please install with:"
    echo "sudo apt install podman"
    exit 1
fi

# This function takes a TARGET as an argument, and builds the library for that target
# It then moves the build artifacts to out/<profile>/<target>/jcan/
function build_target {
    CROSSTARGET="${1}"
    export CROSS_CONTAINER_ENGINE=podman

    # Very important to clean, incase old crates for x86 are present
    cargo clean

    cross build --package jcan --target $CROSSTARGET --release
    cross build --package scripts_postbuild --target $CROSSTARGET --release

    # python build uses a special pyo3 image
    export CARGO=cross
    export CARGO_BUILD_TARGET=${CROSSTARGET}
    export CROSS_CONFIG=${SCRIPT_DIR}/jcan-python/Cross.toml

    cross build --package jcan_python --target $CROSSTARGET --release

    # Run setuptools-rust on jcan_python
    cd jcan-python
    rm -rf ./dist
    rm -rf ./build

    # Change plat-name depending on CROSSTARGET
    if [[ "${CROSSTARGET}" == "aarch64-unknown-linux-gnu" ]];
    then
        PLATNAME="manylinux2014_aarch64"
    elif [[ "${CROSSTARGET}" == "x86_64-unknown-linux-gnu" ]];
    then
        PLATNAME="manylinux2014_x86_64"
    else
        echo "Unknown CROSSTARGET: ${CROSSTARGET}"
        exit 1
    fi    

    python setup.py bdist_wheel --plat-name $PLATNAME --py-limited-api=cp38

    cd ..

    # Copy the resulting wheels to out folder
    mkdir -p out/wheels
    cp -r jcan-python/dist/* out/wheels/
}

# Build for aarch64
build_target "aarch64-unknown-linux-gnu"

# Build for x86_64
build_target "x86_64-unknown-linux-gnu"