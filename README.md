# JorzaCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust, using [cxx-rs](https://cxx.rs/) and [maturin](https://github.com/PyO3/maturin) (a build tool for [pyo3](https://pyo3.rs/)).

> Warning: I have never used Rust before and I don't know what I'm doing

## Feature Status / TODO
 - [x] Blocking send/receive in C++ (`jorzacan.h`) and Python (`jorzacan_python`)
 - [x] `aarch64` build for Jetson TX2
 - [ ] Convenience methods for Frame building, e.g: setting specific bits in a byte, named IDs
 - [x] Replace `maturin` build system with manual scripts, or `setuptools-rust`
 - [x] Rename of `jorzacan_python` to just `jorzacan`
 - [x] Usage examples for C++ and Python
 - [ ] Build an example of JorzaCAN + ROS2 Foxy usage
 - [ ] Implement asyncronous send/receive callback methods
 - [x] Benchmark and speedtest against `python-can` (see `utils/speedtest.sh`, typically speedup is *200%* with `jorzacan`)

## Examples
<details><summary>Python</summary>
<p>

Receive a CAN frame and print it to console

```python
#!/usr/bin/env python
import jorzacan

# This will raise an exception if vcan0 does not exist
bus = jorzacan.Bus("vcan0")

# This will block until a frame is available
f = bus.receive()

print(str(f))
```

</p>
</details>

<details><summary>C++14</summary>
<p>

```cpp
#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jorzacan.h"

using namespace org::jorzacan;

// main function which opens a JorzaBus, creates a JorzaFrame, and sends it!
int main(int argc, char **argv) {
    // Open the CAN bus, will raise an error if vcan0 is not available
    Bus *bus = org::jorzacan::open_bus("vcan0").into_raw();

    // Build a frame
    Frame frame;
    // Both standard and extended IDs are supported!
    frame.id = 0x42;
    // Push bytes into frame from MSB to LSB
    // DLC is automatically calculated
    frame.data.push_back(0x01);
    frame.data.push_back(0x02);
    frame.data.push_back(0x03);
    frame.data.push_back(0x04);

    // Send it!
    bus->send(frame);

    return 0;
}
```

</p>
</details>

## Installation
Download the latest builds from the [Releases Page](https://github.com/leighleighleigh/JorzaCAN/releases), and add them to your include path or install it into your Python environment!

## Quirks / Known Bugs
 - A dedicated `scripts-postbuild` crate is used to move all the build-artifacts (`libjorzacan.a`, `jorzacan.h`, etc...) into `/out/<profile>/<target>/jorzacan`
 - Workspace-level `cargo build` is broken, use `build.sh` instead.
 - C++ examples must be built manually with CMake, their `include` folder is symlinked to the `/out/.../jorzacan` directory

## Development
```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Get code
git clone https://github.com/leighleighleigh/JorzaCAN

# Setup virtual environment
python3 -m venv .venv

# Activate environment and install requirements 
source .venv/bin/activate
pip install -r requirements.txt

## Cross-build scripts
# (This will automatically source .venv/bin/activate if needed)
# (Make sure to follow the prompts to install podman and cross-rs)

./clean.sh
./crossbuild.sh
./release.sh

# Build outputs, including python wheels, can then be found in the ./release folder!

```
