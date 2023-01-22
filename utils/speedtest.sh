#!/usr/bin/env bash

# Compares python-can speed to jcan speed, using candump!

MEASURECMD='candump vcan0 | tqdm --bytes > /dev/null'
RUNTIME=5s

echo "Speedtest comparison of python-can to jCAN"

echo "Running spam-pycan.py for 5s"
timeout $RUNTIME ./spam-pycan.py &
timeout $RUNTIME bash -c "${MEASURECMD}"

echo ""

echo "Running spam-jcan.py for 5s"
timeout $RUNTIME ./spam-jcan.py & 
timeout $RUNTIME bash -c "${MEASURECMD}"

echo ""
