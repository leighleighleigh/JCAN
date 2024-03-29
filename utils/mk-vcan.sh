#!/usr/bin/env bash
#
# Sets up a virtual CAN bus interface, "vcan0"
#
# This is required to pass the unit and integration tests.
#

IFACE=vcan0
[ -n "$1" ] && IFACE=$1

# Load the 'vcan' kernel module

VCAN_LOADED=$(lsmod | grep ^vcan)
if [ -z "${VCAN_LOADED}" ]; then
    if ! sudo modprobe vcan ; then
        printf "Unable to load the 'vcan' kernel module.\n"
        exit 1
    fi
fi

# Add and set up the CAN interface

sudo ip link add type vcan && \
    sudo ip link set up "${IFACE}"


