#include "jcan/include/callback.h"
#include "jcan/src/lib.rs.h"
#include <stdio.h>

namespace leigh { namespace jcan
{
   Bus::Bus() {
      this->jBus = new_jbus().into_raw();
   }

   std::unique_ptr<Bus> new_bus() {
      return std::unique_ptr<Bus>(new Bus());
   }

   void Bus::open(const char *name) {
      // Default queue lengths
      this->jBus->open(name, 2, 256);
   }

   void Bus::open(const char *name, uint16_t tx_queue_len, uint16_t rx_queue_len) {
      this->jBus->open(name, tx_queue_len, rx_queue_len);
   }

   void Bus::close() {
      this->jBus->close();
   }

   void Bus::set_callbacks_enabled(bool mode) {
      this->jBus->set_callbacks_enabled(mode);
   }

   bool Bus::callbacks_enabled() {
      return this->jBus->callbacks_enabled();
   }

   bool Bus::is_open() {
      return this->jBus->is_open();
   }

   void Bus::drop_buffered_frames() {
      return this->jBus->drop_buffered_frames();
   }

   void Bus::set_id_filter(std::vector<uint32_t> allowed_ids) {
      // Convert to a Rust Vec by pushing back each element
      auto rust_vec = rust::Vec<uint32_t>();
      for (auto id : allowed_ids) {
         rust_vec.push_back(id);
      }
      this->jBus->set_id_filter(rust_vec);
   }

   void Bus::set_id_filter_mask(uint32_t allowed, uint32_t allowed_mask) {
      this->jBus->set_id_filter_mask(allowed, allowed_mask);
   }

   void Bus::send(Frame frame) {
      this->jBus->send(frame);
   }

   Frame Bus::receive() {
      return this->jBus->receive();
   }

   Frame Bus::receive_with_timeout(uint64_t timeout_ms) {
      return this->jBus->receive_with_timeout_millis(timeout_ms);
   }

   std::vector<Frame> Bus::receive_from_thread_buffer() {
      std::vector<Frame> stdv;
      auto frames = this->jBus->receive_from_thread_buffer();
      std::copy(frames.begin(), frames.end(), std::back_inserter(stdv));
      return stdv;
   }

  void Bus::add_callback(int id, void (*callback)(Frame)) {
    this->callbacks_[id] = callback;
  }

  void Bus::spin() {
    // If callbacks are not enabled, return immediately
    if (!this->callbacks_enabled()) {
      return;
    }

    // Get a vector of frames from the bus (receive_from_thread_buffer)
    auto frames = this->receive_from_thread_buffer();

    // For each frame, call the callback function associated with the frame's ID
    for (auto frame : frames) {
      auto it = this->callbacks_.find(frame.id);

      // If the ID is not in the map, check if we have an 'any' callback of ID 0 assigned
      if (it != this->callbacks_.end())
      {
        // Call the callback function
        it->second(frame);
      }else{
         // Check if '0' callback exists
         auto it_any = this->callbacks_.find(0);

         if (it_any != this->callbacks_.end())
         {
            // Call the '0' callback
            it_any->second(frame);
         }
      }
    }
  }
}
}
