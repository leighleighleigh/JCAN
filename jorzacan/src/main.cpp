// Example of using the jorzacan library, using headers build by cxx 
#include <stdint.h>
#include <stdio.h>
#include "lib.rs.h"

using namespace org::jorzacan;

/* 
C++-14 example of using the jorzacan library.
*/

// main function which opens a JorzaBus, creates a JorzaFrame, and sends it!
int main(int argc, char **argv) {
    // Open the CAN bus
    // auto bus = org::jorzacan::new_jorzacan_bus("/dev/ttyUSB0");
    JorzaBus *bus = org::jorzacan::new_jorzabus("vcan0").into_raw();

    // Receve a frame
    JorzaFrame frame = bus->receive();

    // Print the frame to stdout, iterating through the data
    printf("Frame: id=0x%x, dlc=%d, data=[", frame.id, frame.dlc);
    for (int i = 0; i < frame.dlc; i++) {
        printf("0x%x", frame.data[i]);
        if (i < frame.dlc - 1) {
            printf(", ");
        }
    }
    printf("]\n");


    return 0;
}