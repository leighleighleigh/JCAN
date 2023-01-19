#include <stdint.h>
#include <jorzacan_t.h>

/* 
C++-11 example of using the jorzacan library.
*/

// main function which opens a JorzaBus, creates a JorzaFrame, and sends it!
int main(int argc, char **argv) {
    // create a new JorzaBus, defining const char *iface = "vcan0"
    const char *iface = "vcan0";
    JorzaBus bus(iface);

    // create a new JorzaFrame
    uint8_t data[8] = {0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
    JorzaFrame frame(0x123, data, 8);

    // send the frame
    bus.send(&frame);

    return 0;
}