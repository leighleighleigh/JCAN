#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jorzacan/src/lib.rs.h"

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

        // Receve a frame
        JorzaFrame frame = bus->receive();

        // Print the frame id, dlc, and data bytes - in candump format!
        printf("%08x#%02x", frame.id, frame.dlc);
        for (int i = 0; i < frame.dlc; i++) {
            printf("%02x", frame.data[i]);
        }
        printf("\n");
    }

    return 0;
}