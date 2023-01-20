// Example of using the jorzacan library, using headers build by cxx 
#include <stdint.h>
#include <stdio.h>
#include <jorzacan/include/demo.h>
#include "jorzacan/src/main.rs.h"

using namespace org::jorzacan;
/* 
C++-11 example of using the jorzacan library.
*/

// main function which opens a JorzaBus, creates a JorzaFrame, and sends it!
int main(int argc, char **argv) {
    // create a new JorzaBus, defining const char *iface = "vcan0"
    JorzaBus bus = jorzabus_open("vcan0");
    JorzaFrame f = bus.receive();

    // printf("Frame: %s", frame.to_string().c_str());

    return 0;
}