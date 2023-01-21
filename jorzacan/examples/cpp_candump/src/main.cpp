#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jorzacan.h"

using namespace org::jorzacan;

/* 
C++-14 example of using the jorzacan library.
*/

// main function which opens a JorzaBus, and prints incoming frames
int main(int argc, char **argv) {
    // Open the CAN bus, and un-boxes it!
    Bus *bus = org::jorzacan::open_bus("vcan0").into_raw();

    // Run forever
    while (1) {

        // Receve a frame
        Frame frame = bus->receive();

        // Print frame using it's to_string method
        // printf("%s\n", frame.to_string().c_str());
    }

    return 0;
}