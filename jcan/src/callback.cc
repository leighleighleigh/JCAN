// Here we define a JCANFrameCallback class, which is a Functor wrapper used to pass C++ functions to Rust.
// Rust can then use this JCANFrameCallback with the execute_callback function (also defined here) to call the C++ function.
// The function must have the signature accepting an argument of org::jcan::Frame and returning void.
// This is a workaround for the fact that Rust can't call C++ functions directly.


#ifndef WRAPPERS_H
#define WRAPPERS_H

#include "callback.h"
// #include "jcan/src/lib.rs.h"

namespace org::jcan
{
    
JCANFrameCallback::JCANFrameCallback () {}

//     public:
//         JCANFrameCallback(void (*callback)(org::jcan::Frame)) {
//             this->callback = callback;
//         }

//         void execute_callback(org::jcan::Frame frame) {
//             this->callback(frame);
//         }
// }

// public static void execute_callback(JCANFrameCallback callback, org::jcan::Frame frame) {
//     callback.execute_callback(frame);
// }

std::unique_ptr<JCANFrameCallback> new_jcan_frame_callback() {
  return std::unique_ptr<JCANFrameCallback>(new JCANFrameCallback());
}

} // namespace org::jcan
#endif