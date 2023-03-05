# JCAN
An easy-to-use SocketCAN library for Python and C++, built in Rust, using [cxx-rs](https://cxx.rs/) and [pyo3](https://pyo3.rs/).

> Warning: I have never used Rust before and I don't know what I'm doing

## Feature Status / TODO
 - [x] Blocking send/receive in C++ (`jcan.h`) and Python (`jcan`)
 - [x] `aarch64` build for Jetson TX2
 - [x] Replace `maturin` build system with manual scripts, or `setuptools-rust`
 - [x] Rename of `jcan_python` to just `jcan`
 - [x] Usage examples for C++ and Python
 - [x] PyPi package release
 - [x] Benchmark and speedtest against `python-can` (see `utils/speedtest.sh`, typically speedup is *200%* with `jcan`)
 - [x] Build an example of JCAN + ROS2 Foxy usage
 - [x] Receive function for specific CAN IDs (e.g `receive_with_id(id : u32)`)
 - [x] Non-blocking receive functions, which return a list of buffered Frames
 - [x] Implement asyncronous send/receive callback methods 
 - [ ] Convenience methods for Frame building, e.g: setting specific bits in a byte, named IDs
 - [ ] TOML-based 'CAN device interface' files, which generate methods like `set_motor_speed(0.5f)` / `set_heater(True)`, etc...

## Installation
*Download the latest builds from the [Releases Page](https://github.com/leighleighleigh/JCAN/releases)! <br>*

For python, it's as easy as...
```bash
pip install jcan
```

For C++, you'll need to download the latest build and add it to your include path manually - check the examples folder for `cmake` usage.

## Examples
> For local development, you can setup a *virtual CAN interface* with the [vcan.sh](https://github.com/leighleighleigh/JCAN/blob/main/utils/vcan.sh) script. <br>
> You can then use the `can-utils` package (`apt install can-utils`) to interact with the `vcan0` interface, on the command line.

<details open><summary>Python</summary>
<p>

Python example showing most of the JCAN features

```python
#!/usr/bin/env python
import jcan
from time import sleep

# This will raise an exception if vcan0 does not exist
bus = jcan.Bus("vcan0")

# This will block until a frame is available
# On another terminal, try sending one with `cansend vcan0 123#0A1B2C3D`
f = bus.receive()
print(str(f))

# This will block until a frame with the matching ID is received
# Behind the scenes, ALL frames are being received, and then ignored if their ID field is different.
# This is inefficient, but simple
f = bus.receive_with_id(0x44)
print(str(f))

# This will set the jcan Bus so that it can ONLY receive frames with these IDs in particular.
# This is more efficient, since the filtering is done at the socket level, rather than by the JBUS library
bus.set_id_filter([0x42,0x66,0x108])

# This will block until a message with one of the above IDs is received
f = bus.receive()
print(str(f))

# Here is how you can send a Frame
f = jcan.Frame(0x12, bytes([1,2,3]))
bus.send(f)

# This is an example of non-blocking receive operation 
# Frames are buffered in the background by the socket, until they are needed
while True:
  print("Checking for frames...")
  frames = bus.receive_nonblocking()
  for f in frames:
    print(str(f))
    
  print("Doing other things...")
  time.sleep(1.0)
```

</p>
</details>

C++ example showing Frame building and sending.

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

> **Lots more examples can be found [in the examples folder](https://github.com/leighleighleigh/JCAN/tree/main/examples)!**

</p>
</details>

## Quirks / Known Bugs
 - A dedicated `scripts-postbuild` crate is used to move all the build-artifacts (`libjcan.a`, `jcan.h`, etc...) into `/out/<profile>/<target>/jcan`
 - Workspace-level `cargo build` is broken, use `crossbuild.sh` instead (as detailed below)
 - C++ examples can be built with the `build.sh` script found in each directory

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
