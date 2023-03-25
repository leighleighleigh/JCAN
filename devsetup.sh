#!/usr/bin/env bash

# This script is used to setup the development environment for the project.
# Basically just installs a .venv 

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# Create a virtual environment
python3 -m venv $SCRIPT_DIR/.venv

# Activate the virtual environment
source $SCRIPT_DIR/.venv/bin/activate

# Install requirements
pip install -r $SCRIPT_DIR/requirements.txt

# Install tools for cross building

# Install rust and cargo!
if ! command -v rustup &> /dev/null
then
    echo "rustup could not be found"
    # Prompt with y/N to install rustup
    read -p "Install rustup? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    else
        echo "rustup not installed, exiting"
        exit 1
    fi
fi

# Check for 'cross' command, else prompt install
if ! command -v cross &> /dev/null
then
    echo "cross could not be found"
    #cargo install cross --git https://github.com/cross-rs/cross
    # Prompt with y/N to install cross
    read -p "Install cross? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        cargo install cross --git https://github.com/cross-rs/cross
    else
        echo "cross not installed, exiting"
        exit 1
    fi
fi

# Check for 'podman' command, else prompt install
if ! command -v podman &> /dev/null
then
    echo "podman could not be found"
    # Prompt with y/N to install podman
    read -p "Install podman? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        sudo apt install podman
    else
        echo "podman not installed, exiting"
        exit 1
    fi
fi

# Install the crates cargo-edit, cargo-get
if ! command -v cargo set-version &> /dev/null
then
    echo "cargo-edit could not be found"
    # Prompt with y/N to install cargo-edit
    read -p "Install cargo-edit? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        cargo install cargo-edit
    else
        echo "cargo-edit not installed, exiting"
        exit 1
    fi
fi

if ! command -v cargo get &> /dev/null
then
    echo "cargo-get could not be found"
    # Prompt with y/N to install cargo-get
    read -p "Install cargo-get? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]
    then
        cargo install cargo-get
    else
        echo "cargo-get not installed, exiting"
        exit 1
    fi
fi


