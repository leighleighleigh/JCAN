#!/usr/bin/env bash

# Removes all VCAN interfaces
# Must have root privleges to run this script

if (( $EUID != 0 )); then
  echo "This script must be run as root"
  exit 1
fi

# Read the vcan interface names, as a multi-line string
RAWIFACES="$(ip link show type vcan | grep 'vcan' | cut -f2 -d':' | cut -f2 -d' ')"

if [ -z "$RAWIFACES" ];
then
 exit 0
fi

# Make an empty array
IFACES=()
# Read the string into this array
readarray -t IFACES<<<$RAWIFACES

# If we have more than 0 items...
if [ ${#IFACES[@]} ]; then

  # If we have an argument from the CLI
  if [ ! -z "$1" ]; then

    # If '-a' or '--all' is provided, delete all interfaces. Else, delete only the last one.
    if [[ "$1" == "--all" || "$1" == "-a" ]]; then
      RM_ALL=1
    fi

    # HELP
    if [[ "$1" == "-h" || "$1" == "--help" ]]; then
      echo "USAGE: ./rm-vcan.sh [-a|--all]"
      echo "Removes the newest (highest-index) VCAN interface by default, optionally removes all interfaces."
      exit 0
    fi
  fi

  # If we have not been asked to delete all VCANs, just delete the last one.
  if [ -z "$RM_ALL" ]; then
    IFACE="${IFACES[${#IFACES[@]}-1]}"
    ip link del ${IFACE}
    echo "Removed ${IFACE}"
  else
    # Otherwise, delete them all!
    for (( i=0; i<${#IFACES[@]}; i++ ))
    do
      IFACE=${IFACES[i]}
      ip link del ${IFACE}
      echo "Removed ${IFACE}"
    done
  fi
else
  exit 0
fi
