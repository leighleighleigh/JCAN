# JorzaCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust, using [cxx-rs](https://cxx.rs/) and [maturin](https://github.com/PyO3/maturin) (a build tool for [pyo3](https://pyo3.rs/)).

> Warning: I have never used Rust before and I don't know what I'm doing

## Feature Status / TODO
 - [x] Blocking send/receive in C++ (`jorzacan.h`) and Python (`jorzacan_python`)
 - [ ] `aarch64` build for Jetson TX2
 - [ ] Convenience methods for Frame building, e.g: setting specific bits in a byte, named IDs
 - [ ] Replace `maturin` build system with manual scripts, or `setuptools-rust`
 - [ ] Rename of `jorzacan_python` to just `jorzacan`
 - [ ] Documented examples for C++ and Python, built with `cmake-rs` and `setuptools-rust`
 - [ ] Build an example of JorzaCAN + ROS2 Foxy usage
 - [ ] Implement asyncronous send/receive callback methods
 - [ ] Benchmark and speedtest against `python-can` and `socketcan.cpp`

## Quirks / Known Bugs
 - A dedicated `scripts-postbuild` crate is used to move all the build-artifacts (`libjorzacan.a`, `jorzacan.h`, etc...) into `/out/<profile>/<target>/jorzacan`
 - Workspace-level `cargo build` is broken, use `build.sh` instead.
 - C++ examples must be built manually with CMake, their `include` folder is symlinked to the `/out/.../jorzacan` directory

## Installation
Download the latest builds from the [Releases Page](https://github.com/leighleighleigh/JorzaCAN/releases), and add them to your include path or install it into your Python environment!

## Development
```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Get code
git clone https://github.com/leighleighleigh/JorzaCAN

# Setup virtual environment (REQUIRED for maturin build tool)
python3 -m venv .venv

# Activate environment and install requirements 
source .venv/bin/activate
pip install -r requirements.txt

## Build (debug/manual)
# ... to build the base JorzaCAN library
cd jorzacan
cargo build

# ... to move the build outputs to the correct location
cd scripts-postbuild
cargo build

# ... to build the python bindings, and install it to the virtual environment
cd jorzacan-python
maturin develop


## Build (release mode)
# This will automatically source .venv/bin/activate if needed
./build.sh
# Install and use!
# ... for Python
pip install ./out/wheels/jorzacan_python-0.1.0-cp38-abi3-manylinux_2_34_x86_64.whl
# ... for C++14 add this path to the includes list,
#     and then '#include <jorzacan.h>'
./out/release/x86_64-unknown-linux-gnu/jorzacan 

```
