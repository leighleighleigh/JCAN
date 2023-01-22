#!/usr/bin/env python
import jcan
from time import sleep

bustype = 'socketcan'
channel = 'vcan0'

bus = jcan.Bus(channel)

# Set filter to only ids 0x42 and 0x66
bus.set_id_filter([0x42,0x66])

# This will ONLY print frames that match the ID, and will not block!
# Unhandles messages are buffered by the SocketCAN socket.
while True:
    frames = bus.receive_nonblocking()

    for f in frames:
        print(str(f))
        # Simulated delay
        sleep(0.01)

    print("Not blocking!!!")


