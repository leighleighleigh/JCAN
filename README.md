# JCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust, using [cxx-rs](https://cxx.rs/) and [pyo3](https://pyo3.rs/).

> Warning: I have never used Rust before and I don't know what I'm doing

## Feature Status / TODO
 - [x] Blocking send/receive in C++ (`jcan.h`) and Python (`jcan`)
 - [x] `aarch64` build for Jetson TX2
 - [x] Replace `maturin` build system with manual scripts, or `setuptools-rust`
 - [x] Rename of `jcan_python` to just `jcan`
 - [x] Usage examples for C++ and Python
 - [x] Benchmark and speedtest against `python-can` (see `utils/speedtest.sh`, typically speedup is *200%* with `jcan`)
 - [ ] Build an example of JCAN + ROS2 Foxy usage
 - [ ] Receive function for specific CAN IDs (e.g `receive_filtered(id : u32)`)
 - [ ] Implement asyncronous send/receive callback methods 
 - [ ] Convenience methods for Frame building, e.g: setting specific bits in a byte, named IDs
 - [ ] TOML-based 'CAN device interface' files, which generate methods like `set_motor_speed(0.5f)` / `set_heater(True)`, etc...
 

## Examples
> NOTE: For local development, you can setup a *virtual CAN interface* with the [vcan.sh](https://github.com/leighleighleigh/JCAN/blob/main/utils/vcan.sh) script. <br>
> You can then use the `can-utils` package (`apt install can-utils`) to interact with the `vcan0` interface, on the command line.

<details open><summary>Python</summary>
<p>

Receive a CAN frame and print it to console

```python
#!/usr/bin/env python
import jcan

# This will raise an exception if vcan0 does not exist
bus = jcan.Bus("vcan0")

# This will block until a frame is available
# On another terminal, try sending one with `cansend vcan0 123#0A1B2C3D`
f = bus.receive()

print(str(f))
```

</p>
</details>

<details open><summary>C++14</summary>
<p>

```cpp
#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan.h"

using namespace org::jcan;

// main function which opens a JBus, creates a JFrame, and sends it!
int main(int argc, char **argv) {
    // Open the CAN bus, will raise an error if vcan0 is not available
    Bus *bus = org::jcan::open_bus("vcan0").into_raw();

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
    
    // Open another terminal and type `candump vcan0`, then run this program again!

    return 0;
}
```
More examples can be found [in the examples folder](https://github.com/leighleighleigh/JCAN/tree/main/examples).

</p>
</details>

## Installation
Download the latest builds from the [Releases Page](https://github.com/leighleighleigh/JCAN/releases), and add them to your include path or install it into your Python environment!

## Quirks / Known Bugs
 - A dedicated `scripts-postbuild` crate is used to move all the build-artifacts (`libjcan.a`, `jcan.h`, etc...) into `/out/<profile>/<target>/jcan`
 - Workspace-level `cargo build` is broken, use `build.sh` instead.
 - C++ examples must be built manually with CMake, their `include` folder is symlinked to the `/out/.../jcan` directory

## Development
```bash
# Get code
git clone https://github.com/leighleighleigh/JCAN

# Setup the build environment, which
# - Installs rust 
# - Installs cross-rs
# - Installs podman
# - Sets up a python virtual environment in the repo, under .venv
./devsetup.sh

## Run the build scripts!
# (This will automatically source .venv/bin/activate if needed)

./clean.sh
./crossbuild.sh
./release.sh

# Build outputs, including python wheels, can then be found in the ./release folder!

```
