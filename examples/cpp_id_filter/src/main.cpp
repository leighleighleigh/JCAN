#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan.h"

using namespace org::jcan;

/* 
C++-14 example of using the jcan library.
*/

// main function which opens a JBus, and prints incoming frames
int main(int argc, char **argv) {
    // Open the CAN bus, and un-boxes it!
    Bus *bus = org::jcan::open_bus("vcan0").into_raw();

    // Set the bus ID filter to 0x42 and 0x66 only
    bus->set_id_filter({0x42, 0x66});
    
    // Run forever
    while (1) {
        // Receve a frame (blocking)
        Frame frame = bus->receive();
        // Print frame using it's to_string method
        printf("%s\n", frame.to_string().c_str());
    }

    return 0;
}