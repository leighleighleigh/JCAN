#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan/jcan.h"

using namespace leigh::jcan;

/* 
A basic example of sending and recieving CAN Frames with JCAN
*/

int main(int argc, char **argv) {
    // Build new Bus object, which is a unique pointer to a Bus object
    std::unique_ptr<Bus> bus = new_bus();

    // Open the bus
    bus->open("vcan0");

    // Loop forever, sending frames and printing the ones we recieve
    unsigned char i = 0;

    while(true)
    {
        i++;
        Frame frameToSend = new_frame(0x200, {i,uint8_t(i/10),uint8_t(i/100),uint8_t(i%2)});
        printf("Sending: %s...\n",frameToSend.to_string().c_str());
        bus->send(frameToSend);

        printf("Waiting 10 seconds for any frame...\n");

        try
        {
            Frame frameReceived = bus->receive_with_timeout(10000);
            printf("Received: %s\n", frameReceived.to_string().c_str());
        }
        catch(const std::exception& e)
        {
            printf("Timeout: %s\n", e.what());
        }
    }

    return 0;
}
