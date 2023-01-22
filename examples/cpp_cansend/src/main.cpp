#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan.h"

using namespace org::jcan;

/* 
C++-14 example of using the jcan library.
*/

// main function which opens a JBus, creates a JFrame, and sends it!
int main(int argc, char **argv) {
    // Open the CAN bus
    Bus *bus = org::jcan::open_bus("vcan0").into_raw();

    // Run forever
    while (1) {

        // Send a frame
        Frame frame;
        frame.id = 0x42;
        frame.data.push_back(0x01);
        frame.data.push_back(0x02);
        frame.data.push_back(0x03);
        frame.data.push_back(0x04);

        bus->send(frame);
    }

    return 0;
}