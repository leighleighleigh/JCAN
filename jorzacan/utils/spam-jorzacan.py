#!/usr/bin/env python
import jorzacan

bustype = 'socketcan'
channel = 'vcan0'

bus = jorzacan.PyCanSocket(channel)

while True:
    bus.send(0x123, bytes([0xD,0xE,0xA,0xD,0xB,0xE,0xE,0xF]))

