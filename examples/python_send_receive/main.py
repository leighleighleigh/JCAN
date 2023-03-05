#!/usr/bin/env python
import jcan
from ctypes import c_uint8

if __name__ == "__main__":
    bus = jcan.Bus()

    # Set a filter, from list...
    # bus.set_id_filter([0x1A0,0x1A1,0x1A2,0x1A3])

    # .. or from a mask!
    bus.set_id_filter_mask(0x1A0, 0xFF0)

    bus.open("vcan0")

    i = 0

    while True:
        i += 1

        # The list of values will be cast to uint8's by JCAN library - so be careful to double check the values!
        frameToSend = jcan.Frame(0x200, [i,i/10,i/100,i%2,i%3,i%4,i%5,i*10])
        print(f"Sending {frameToSend}")
        bus.send(frameToSend)

        print("Waiting for frame...")
        frameReceived = bus.receive()
        print(f"Received ID: {frameReceived.id}, DATA: {frameReceived.data}")


