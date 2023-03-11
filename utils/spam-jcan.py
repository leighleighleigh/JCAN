#!/usr/bin/env python
import jcan
import time

channel = 'vcan0'

bus = jcan.Bus()
bus.open(channel)

while True:
    f = jcan.Frame(0x1A3, [0xC,0x0,0xF,0xF,0xE,time.time_ns() % 64,time.time_ns() % 255])
    bus.send(f)
    f = jcan.Frame(0x1A5, [])
    bus.send(f)

