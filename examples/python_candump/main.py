#!/usr/bin/env python
import jcan

bustype = 'socketcan'
channel = 'vcan0'

bus = jcan.Bus(channel)

while True:
    f = bus.receive()
    print(str(f))


