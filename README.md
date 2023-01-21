# JorzaCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust.

> Warning: I have never used Rust before and I don't know what I'm doing

## Quickstart

```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Get code
git clone https://github.com/leighleighleigh/JorzaCAN

# Build (release mode)
./build.sh

# Install and use!
# ... for Python
pip install ./target/wheels/jorzacan_python..._.wheel

# ... for C++14 add this path to the includes list,
#     and then '#include <jorzacan.h>'
./out/release/x86_64-unknown-linux-gnu/jorzacan 

```

## Feature Status / TODO
 - [x] Blocking send/receive in C++ (`jorzacan.h`) and Python (`jorzacan_python`)
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

