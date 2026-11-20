#!/usr/bin/env bash

if grep -qw 'archoly\.noinstall' /proc/cmdline; then
  echo "archoly.noinstall set; starting normal live shell (run /do_setup.sh to install manually)."
  exec bash -l
fi

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
