#!/usr/bin/env python
import jcan

bustype = 'socketcan'
channel = 'vcan0'

bus = jcan.Bus(channel)

# Set filter to only ids 0x42 and 0x66
bus.set_id_filter([0x42,0x66])

# This will ONLY print frames that match the ID
while True:
    f = bus.receive()
    print(str(f))


