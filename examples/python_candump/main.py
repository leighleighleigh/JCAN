#!/usr/bin/env python
import jorzacan

bustype = 'socketcan'
channel = 'vcan0'

bus = jorzacan.Bus(channel)

while True:
    f = bus.receive()
    print(str(f))


