#!/usr/bin/env python
import time
import can

bustype = 'socketcan'
channel = 'vcan0'

bus = can.Bus(channel=channel, interface=bustype)

while True:
    msg = can.Message(arbitration_id=0x123, data=[0xD,0xE,0xA,0xD,0xB,0xE,0xE,0xF], is_extended_id=False)
    bus.send(msg)

