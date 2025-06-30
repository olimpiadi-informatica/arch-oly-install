#!/usr/bin/env bash

set -e

DISK=
SZ=0

for d in /dev/nvme0n1 /dev/nvme0n2 /dev/nvme1n1 /dev/nvme1n2 /dev/sda /dev/sdb
do
  sz=$(sfdisk -s $d 2> /dev/null)
  if [ "$SZ"0 -lt "$sz"0 ]
  then
    DISK=$d
    SZ=$sz
  fi
done

echo "Installing on disk $DISK (size: $((SZ/1024/1024)) GB)."

export DISK=$DISK

for script in $(ls /pre_install/*)
do
  echo -e "\033[32;1mPre-install $script\033[;m"
  bash $script
done

reboot
