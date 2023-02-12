#include "jcan/include/callback.h"
#include "jcan/src/lib.rs.h"
#include <stdio.h>

namespace org::jcan
{
   void hello()
   {
        printf("Hello!");
   } 

   Bus::Bus() {
      this->jBus = new_jbus().into_raw();
   }

   std::unique_ptr<Bus> new_bus() {
      return std::unique_ptr<Bus>(new Bus());
   }

   void Bus::open(const char *name) {
      this->jBus->open(name);
   }

   void Bus::set_id_filter(std::vector<uint32_t> allowed_ids) {
      // Convert to a Rust Vec by pushing back each element
      auto rust_vec = rust::Vec<uint32_t>();
      for (auto id : allowed_ids) {
         rust_vec.push_back(id);
      }
      this->jBus->set_id_filter(rust_vec);
   }

   void Bus::send(Frame frame) {
      this->jBus->send(frame);
   }

   void Bus::receive() {
      this->jBus->receive();
   }

   // void Bus::add_callback(std::function<void(Frame)> callback) {
   //    // this->jBus->add_callback(callback);
   //    // TODO
   // }

   std::vector<Frame> Bus::receive_many() {
      std::vector<Frame> stdv;
      auto frames = this->jBus->receive_many();
      std::copy(frames.begin(), frames.end(), std::back_inserter(stdv));
      return stdv;
   }
}