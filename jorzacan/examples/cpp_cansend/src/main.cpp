#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jorzacan.h"

using namespace org::jorzacan;

/* 
C++-14 example of using the jorzacan library.
*/

// main function which opens a JorzaBus, creates a JorzaFrame, and sends it!
int main(int argc, char **argv) {
    // Open the CAN bus
    Bus *bus = org::jorzacan::open_bus("vcan0").into_raw();

    // Run forever
    while (1) {

        // Send a frame
        JorzaFrame frame;
        frame.id = 0x42;
        frame.dlc = 4;
        frame.data.push_back(0x01);
        frame.data.push_back(0x02);
        frame.data.push_back(0x03);
        frame.data.push_back(0x04);
        assert(frame.data.size() == frame.dlc);

        bus->send(frame);
    }

    return 0;
}