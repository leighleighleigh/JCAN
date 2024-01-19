# Below are some hand-crafted stub-types for the JCAN python package.
# This means you get nice auto-completion and type-analysis, even though it's a static library.
# This file is based off the contents of ./jcan_python/src/lib.rs
from typing import List, Optional, Self, Callable, Union

class Frame:
    def __init__(self, id: int, data: List[Union[int,float]]) -> None: ...
    """
    :param: id: The 11-bit CAN ID of the frame.
    :param: data: The data bytes of the frame, which will be cast to uint8's by JCAN library - so be careful to double check the values!
    """
    def __str__(self) -> str: ...
    def id(self) -> int: ...
    """
    :return: The 11-bit CAN ID of the frame.
    """
    def data(self) -> List[int]: ...
    """
    :return: The data bytes of the frame.
    """

class Bus:
    def __init__(self) -> None: ...
    def open(self, interface: str, tx_queue_len: int = 2, rx_queue_len: int = 256) -> None: ...
    """
    :param: interface: The name of the CAN interface to open, e.g. "vcan0".
    :param: tx_queue_len: The length of the internal transmit queue, after which send() will block.
    :param: rx_queue_len: The length of the internal receive queue, after which older Frames will be dropped.
    """
    def close(self) -> None: ...
    """
    Closes the CAN interface.
    """
    def is_open(self) -> bool: ...
    """
    :return: True if the CAN interface is open, False otherwise.
    """
    def callbacks_enabled(self) -> bool: ...
    """
    :return: True if callbacks are enabled, False otherwise.
    """
    def set_callbacks_enabled(self, mode: bool) -> None: ...
    """
    :param: mode: True to enable callbacks, False to disable.
    """
    def receive(self) -> Frame: ...
    """
    Blocks until a frame is received, then returns it.
    :return: The received frame.
    """
    def receive_with_timeout(self, timeout_ms: int) -> Frame: ...
    """
    Blocks until a frame is received, or the timeout expires, then returns it.
    :param: timeout_ms: The timeout in milliseconds.
    :return: The received frame, or None if the timeout expired.
    """
    def send(self, frame: Frame) -> None: ...
    """
    Sends a frame, blocking until it is queued for transmission on the TX queue.
    :param: frame: The frame to send.
    """
    def drop_buffered_frames(self) -> None: ...
    """
    Drop all frames in the RX queue.
    """
    def set_id_filter(self, allowed_ids: List[int]) -> None: ...
    """
    Set a filter for the CAN ID's that will be received, and for which callbacks will be called.
    :param: allowed_ids: A list of allowed CAN ID's.
    """
    def set_id_filter_mask(self, allowed: int, allowed_mask: int) -> None: ...
    """
    Set a filter for the CAN ID's that will be received, and for which callbacks will be called.
    :param: allowed: The base allowed ID value.
    :param: allowed_mask: The mask for bits of the allowed ID, which are to be checked.
    """
    def receive_from_thread_buffer(self) -> List[Frame]: ...
    """
    :return: A list of frames that are currently in the internal receive buffer.
    """
    def add_callback(self, frame_id: int, callback: Callable[[Frame], None]) -> None: ...
    """
    Add a callback for a specific CAN ID.
    :param: frame_id: The CAN ID for which the callback will be called.
    :param: callback: The callback function, which will be called with the received frame as an argument.
    """
    def spin(self) -> None: ...
    """
    Needs to be called periodically to process received frames and call callbacks.
    """

