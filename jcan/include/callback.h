// include/callback.h

#pragma once

// #include <rust/cxx.h>
#include <memory>

namespace org::jcan
{

// This is a forward declaration of the Frame struct, which is defined in the Rust library
struct Frame;

// This has the same signature as the callback function in the Rust library
using FrameCallback = void(*)(Frame frame);

// This executes a FrameCallback
void execute_callback(FrameCallback callback, Frame frame);
}