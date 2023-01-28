#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan/jcan.h"

using namespace org::jcan;

/* 
C++-14 example of using the jcan library.
*/

// main function which opens a JBus, and prints incoming frames
int main(int argc, char **argv) {
    // Open the CAN bus, and un-boxes it!
    Bus *bus = new_bus().into_raw();

    // Set the bus ID filter to 0x42 and 0x66 only
    bus->set_id_filter({0x42, 0x66});

    // Open the bus
    bus->open("vcan0");

    // Run forever
    while (1) {
        bus->spin();
        auto frame = bus->receive();

        // auto hides all the type-casting crimes, thankyou auto
        printf("%s", frame.to_string().c_str());
    }

    return 0;
}