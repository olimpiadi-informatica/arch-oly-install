#!/usr/bin/env bash

if [ -f /setup_already_attempted ]
then
  echo "Setup script restarted? Starting recovery shell"
  bash
fi

touch /setup_already_attempted

trap '' INT 
bash /do_setup.sh
trap INT

echo "Setup script failed, starting recovery shell"

bash
