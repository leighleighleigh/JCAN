#!/usr/bin/env python
import jcan
import time

channel = 'vcan0'

bus = jcan.Bus()

def recv_handler(frame):
    print(frame)

# add_callback of ID 0 will catch all un-handled frames :)
bus.add_callback(0x0, recv_handler)
bus.open(channel)

while True:
    bus.spin()
    time.sleep(0.01)


