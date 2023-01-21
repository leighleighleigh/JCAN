# JorzaCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust.

> Warning: I have never used Rust before and I don't know what I'm doing

## Status
 - Blocking read/write in both C++ and Python is functional!
 - Workspace-level cargo-build is somewhat broken, individual `cargo build`s or `maturin build` is needed for each package.
 - `scripts-postbuild` is used to move all the build-artifacts (`libjorzacan.a`, `jorzacan.h`, etc...) into `/out/<profile>/<target>/jorzacan`
 - C++ examples must be built manually with CMake, their `include` folder is symlinked to the `/out/.../jorzacan` directory

