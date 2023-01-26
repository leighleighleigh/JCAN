#include "jcan/src/lib.rs.h"
#include "jcan/include/callback.h"


namespace org::jcan
{
    // This executes a FrameCallback
    void execute_callback(FrameCallback callback, Frame frame)
    {
        callback(frame);
    }
}