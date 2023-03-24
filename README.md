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
import time

if __name__ == "__main__":
    bus = jcan.Bus()

    # Set a filter, from list...
    # bus.set_id_filter([0x1A0,0x1A1,0x1A2,0x1A3])

    # .. or from a mask!
    bus.set_id_filter_mask(0x1A0, 0xFF0)

    # This is our callback function for new frames
    def on_frame_five(frame : jcan.Frame):
        print(f"FRAME 1A5: {frame.data}")

    def on_frame_d(frame : jcan.Frame):
        print(f"FRAME 1AD {frame.data}")
        # print(frame.data[0])

    bus.add_callback(0x1A5, on_frame_five)
    bus.add_callback(0x1AD, on_frame_d)

    bus.open("vcan0")

    while True:
        # The list of values will be cast to uint8's by JCAN library - so be careful to double check the values!
        # frameToSend = jcan.Frame(0x200, [time.time()%255, (time.time()*1000)%255])
        # print(f"Sending {frameToSend}")
        # bus.send(frameToSend)

        # Spin is required for our callbacks to be processed.
        # Make sure .spin is called from your MAIN THREAD
        bus.spin()

        # bus.spin is non-blocking if nothing is there - resulting in a 'busy' loop
        # this sleep is to prevent that. In your code, you will probably be doing more important things here!
        time.sleep(0.01)

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
#include "jcan/jcan.h"

using namespace org::jcan;

/* 
A basic example of sending and recieving CAN Frames with JCAN
*/

int main(int argc, char **argv) {
    // Build new Bus object, which is a unique pointer to a Bus object
    std::unique_ptr<Bus> bus = new_bus();

    // Set ID filter using a vector of allowed IDs we can receive
    // std::vector<uint32_t> allowed_ids = {0x100, 0x123, 0x456, 0x789};
    // bus->set_id_filter(allowed_ids);

    // We can also also set a mask of allowed IDs
    // The filter below will only accept frames who's ID is 0x1A#, where '#' can be anything.
    // Combinations of base+mask can be used to make a very flexible filter.. but it can get quite confusing, too!
    // The format used below, of the form 'base_id + part of ID we don't care about',
    // is a nice simple way to use this feature.
    bus->set_id_filter_mask(0x1A0,0xFF0);

    // Open the bus
    bus->open("vcan0");

    // Loop forever, sending frames and printing the ones we recieve
    unsigned char i = 0;

    while(true)
    {
        i++;
        Frame frameToSend = new_frame(0x200, {i,i/10,i/100,i%2,i%3,i%4,i%5,i*10});
        printf("Sending: %s...\n",frameToSend.to_string().c_str());
        bus->send(frameToSend);

        printf("Waiting for frame...\n");
        Frame frameReceived = bus->receive();
        printf("Received: %s\n", frameReceived.to_string().c_str());
    }

    return 0;
}
```

> **Lots more examples can be found [in the examples folder](https://github.com/leighleighleigh/JCAN/tree/main/examples)!**

</p>
</details>

## Quirks / Known Bugs
 - A dedicated `scripts-postbuild` crate is used to move all the build-artifacts (`libjcan.a`, `jcan.h`, etc...) into `/out/<profile>/<target>/jcan`
 - Workspace-level `cargo build` is broken, use `crossbuild.sh` instead (as detailed below)
 - C++ examples can be built with `make`, which uses the `Makefile` in each directory to run `cmake` for you

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
