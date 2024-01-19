#!/usr/bin/env python3

# pytest tests for JCAN
# Leigh Oliver, January 2024 
# (I finished my degree last year <3)

import pytest
from jcan import Bus, Frame

def test_open_bus():
    bus = Bus()
    bus.open("vcan0")
    bus.close()

def test_send_receive():
    bus = Bus()
    bus.open("vcan0")

    frame = Frame(0x100, [1,2,3,4,5,6,7,8])
    bus.send(frame)