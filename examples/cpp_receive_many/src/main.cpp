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
    // Build new Bus object, which is a unique pointer to a Bus object
    std::unique_ptr<Bus> bus = new_bus();
  
    // Set ID filter using a vector of allowed IDs
    std::vector<uint32_t> allowed_ids = {0x123, 0x456, 0x789};
    bus->set_id_filter(allowed_ids);

    // Open the bus
    bus->open("vcan0");
    
    // Say hello (it's only polite)
    hello();

    // Run forever
    while (1) {
        // Receve a frame
        auto frames = bus->receive_many();

        // Print frames
        for(auto frame : frames){
          // Print frame using it's to_string method
          printf("%s\n", frame.to_string().c_str());
        }
    }

    return 0;
}
