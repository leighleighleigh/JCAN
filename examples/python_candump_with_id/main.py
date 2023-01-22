#!/usr/bin/env python
import jcan

bustype = 'socketcan'
channel = 'vcan0'

bus = jcan.Bus(channel)

while True:
    print("Waiting for frame with ID 0x42")
    f = bus.receive_with_id(0x42)
    print(str(f))


