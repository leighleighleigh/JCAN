#!/usr/bin/env python
import jcan

bustype = 'socketcan'
channel = 'vcan0'

bus = jcan.Bus(channel)

while True:
    f = jcan.Frame(0x123, bytes([0xD,0xE,0xA,0xD,0xB,0xE,0xE,0xF]))
    bus.send(f)

