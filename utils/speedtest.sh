#!/usr/bin/env bash

# Compares python-can speed to jorzacan speed, using candump!

MEASURECMD='candump vcan0 | tqdm --bytes > /dev/null'
RUNTIME=5s

echo "Speedtest comparison of python-can to JorzaCAN"

echo "Running spam-pycan.py for 5s"
timeout $RUNTIME ./spam-pycan.py &
timeout $RUNTIME bash -c "${MEASURECMD}"

echo ""

echo "Running spam-jorzacan.py for 5s"
timeout $RUNTIME ./spam-jorzacan.py & 
timeout $RUNTIME bash -c "${MEASURECMD}"

echo ""
