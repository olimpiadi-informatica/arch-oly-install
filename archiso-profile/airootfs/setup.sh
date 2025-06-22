#!/usr/bin/env bash

trap '' INT 
bash /do_setup.sh
trap INT

echo "Setup script failed, starting recovery shell"

exec bash
