#include <stdint.h>
#include <stdio.h>
#include <vector>
#include "jcan/jcan.h"
#include <unistd.h>
using namespace org::jcan;

/* 
C++-14 example of using the jcan library with callbacks.
*/

// Example of handling a frame within a class object
class MyFrameHandler {
public:
    void on_frame(Frame frame) {
        printf("MFH Received frame: %s\n", frame.to_string().c_str());
    }
}; 

void on_frame(Frame frame) {
    printf("Received frame: %s\n", frame.to_string().c_str());
}

// main function which opens a JBus, and prints incoming frames
int main(int argc, char **argv) {
    // Build new Bus object, which is a unique pointer to a Bus object
    std::unique_ptr<Bus> bus = new_bus();
  
    // Set ID filter using a vector of allowed IDs
    std::vector<uint32_t> allowed_ids = {0x100, 0x123, 0x456, 0x789};
    bus->set_id_filter(allowed_ids);

    // Instantiate a frame handler object
    MyFrameHandler frame_handler;

    // Add a callback functions 
    bus->add_callback(0x123, &on_frame);
    bus->add_callback(0x456, &on_frame);
    bus->add_callback(0x789, &on_frame);

    // Add a callback function to a class object
    bus->add_callback_to(0x100, &frame_handler, &MyFrameHandler::on_frame);

    // Open the bus
    bus->open("vcan0");

    // Run forever
    while (1) {
        // Send a frame every 1 second
        Frame frameToSend = new_frame(0x200, {1,2,3,4,5,6,7,8});
        printf("Sending: %s...\n",frameToSend.to_string().c_str());
        bus->send(frameToSend);
        sleep(1);
        printf("Spinning...\n");
        bus->spin();
    }

    return 0;
}
