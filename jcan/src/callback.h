// include/callback.h

#pragma once
#include "rust/cxx.h"
// #include "src/lib.rs.h"
#include <memory>

namespace org::jcan {
class JCANFrameCallback {
public:
  JCANFrameCallback();
};

std::unique_ptr<JCANFrameCallback> new_jcan_frame_callback();
}