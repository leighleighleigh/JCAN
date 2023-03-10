#!/usr/bin/env python
import jcan
import time

channel = 'vcan1'

bus = jcan.Bus()
bus.open(channel)

while True:
    f = jcan.Frame(0x1A3, [0xC,0x0,0xF,0xF,0xE,0xE,time.time_ns()])
    bus.send(f)

