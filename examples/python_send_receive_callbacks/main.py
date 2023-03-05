#!/usr/bin/env python
import jcan
import time

if __name__ == "__main__":
    bus = jcan.Bus()

    # Set a filter, from list...
    # bus.set_id_filter([0x1A0,0x1A1,0x1A2,0x1A3])

    # .. or from a mask!
    bus.set_id_filter_mask(0x1A0, 0xFF0)

    # This is our callback function for new frames
    def on_frame_five(frame : jcan.Frame):
        print(f"FRAME 1A5: {frame.data}")

    def on_frame_d(frame : jcan.Frame):
        print(f"FRAME 1AD {frame.data}")
        # print(frame.data[0])

    bus.add_callback(0x1A5, on_frame_five)
    bus.add_callback(0x1AD, on_frame_d)

    bus.open("vcan0")

    while True:
        # The list of values will be cast to uint8's by JCAN library - so be careful to double check the values!
        # frameToSend = jcan.Frame(0x200, [time.time()%255, (time.time()*1000)%255])
        # print(f"Sending {frameToSend}")
        # bus.send(frameToSend)

        # Spin is required for our callbacks to be processed.
        # Make sure .spin is called from your MAIN THREAD
        bus.spin()

        # bus.spin is non-blocking if nothing is there - resulting in a 'busy' loop
        # this sleep is to prevent that. In your code, you will probably be doing more important things here!
        time.sleep(0.01)





