#!/usr/bin/env python
import jcan
import time

if __name__ == "__main__":
    bus = jcan.Bus()

    # set_id_filter_mask(BASE, MASK)
    # MASK sets the REQUIRED bits in BASE, in order to accept a frame.
    bus.set_id_filter_mask(0x1A0, 0xFF0)

    """
    FRAME CALLBACK FUNCTIONS
    """

    def on_frame_five(frame : jcan.Frame):
        print(f"FRAME 1A5: {frame.data}")

    def on_frame_d(frame : jcan.Frame):
        print(f"FRAME 1AD {frame.data}")
        # print(frame.data[0])

    def on_anyother_frame(frame: jcan.Frame):
        print(f"FRAME {frame.id}: {frame.data}")

    bus.add_callback(0x1A5, on_frame_five)
    bus.add_callback(0x1AD, on_frame_d)

    # The add_callback for ID zero will be called when any other frame is received,
    # as long as that frame does not have a callback associated with it.
    bus.add_callback(0x0, on_anyother_frame)

    """
    BLOCKING INITIALISATION SECTION
    """
    
    # Open the bus
    bus.open("vcan0")
    
    # INITIALISATION EXAMPLE !
    # You might want to check some devices exist on the bus,
    # and it's usually easier to write that with 'blocking' code.

    # You can absolutely do that, BUT, make sure to DISABLE CALLBACKS FIRST!
    # Otherwise, many frames may 'queue up' in the buffer, while you fiddle around
    # with the blocking functions in this section.
    bus.set_callbacks_enabled(False)

    # example: wait indefinitely for frame '0x1A1'
    print("Waiting for frame ID 0x1A1")
    while True: # while loop is because our ID mask is quite wide, so we need to check we got the right one!
      start_frame : jcan.Frame = bus.receive()
      if start_frame.id == 0x1A1:
        print(start_frame)
        break

    # then, send a 'hello' command, and wait 10 seconds for a response '0x1A2'
    print("Sending frame ID 0x1A2")
    bus.send(jcan.Frame(0x1A2, [1,2,3,4,5,6,7]))

    # e.g: wait for a 'ready' frame, 0x1A3
    # if no reply, print an error message, else print the frame.
    print("Waiting 10s for frame 0x1A3...")
    while True:
      try:
        ready_frame : jcan.Frame = bus.receive_with_timeout(10000)

        if ready_frame.id == 0x1A3:
          print(ready_frame)
          break
      except OSError as e:
        # OSError number 110 means timeout!
        print("Ready message not received in 10 seconds!")

    bus.set_callbacks_enabled(True)

    """
    CONTINUE WITH CALLBACKS ENABLED
    """

    while True:
        # Spin is required for our callbacks to be processed.
        # Make sure .spin is called from your MAIN THREAD
        bus.spin()

        # bus.spin is non-blocking if nothing is there - resulting in a 'busy' loop
        # this sleep is to prevent that. In your code, you will probably be doing more important things here!
        time.sleep(0.1)





