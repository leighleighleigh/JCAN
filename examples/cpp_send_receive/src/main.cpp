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
