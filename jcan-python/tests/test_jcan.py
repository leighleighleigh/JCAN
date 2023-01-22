import jcan
import pytest

def test_jcan_bus_open_failure():
    with pytest.raises(IOError):
        channel = "vcan99"
        bus = jcan.Bus(channel)

def test_jcan_standard_frame():
    try:
        print("Creating standard frame with ID 0x123")
        f = jcan.Frame(0x123, bytes([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]))
        print(f)
        assert True 
    except:
        assert False

def test_jcan_ext_frame():
    try:
        print("Creating extended frame with ID 0xF00123")
        f = jcan.Frame(0xF00123, bytes([0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]))
        print(f)
        assert True 
    except:
        assert False

def test_jcan_standard_frame_large():
    with pytest.raises(IOError):
        f = jcan.Frame(0x123, bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAA]))

def test_jcan_standard_frame_empty():
    with pytest.raises(IOError):
        f = jcan.Frame(0x200, bytes([]))