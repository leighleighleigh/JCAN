#pragma once

// #include <rust/cxx.h>
#include <memory>
#include <vector>

namespace org::jcan
{

// This is a forward declaration of the Frame struct, which is defined in the Rust library
struct Frame;

// This is a forward declaration of the JBus class, which is defined in the Rust library.
// While we cannot inherit from this class, we can intantiate it within a Bus 'wrapper class'.
// This will allow us to add the c++-specific add_callback functionality around the Rust implementation,
// in a clean way.
class JBus;

void hello();

class Bus{
private:
    JBus *jBus; //pointer to a JBus object

public:
  Bus();
  void open(const char *name);
  
  void set_id_filter(std::vector<uint32_t> allowed_ids);

  void send(Frame frame);
  void receive();

  // void add_callback(std::function<void(Frame)> callback);
  std::vector<Frame> receive_many();
};

std::unique_ptr<Bus> new_bus();
}