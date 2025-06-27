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
echo "If this is wrong, you have 60 seconds to stop this installation."
for i in 60 50 40 30 20 10
do
  echo -n "$i... "
  sleep 10
done
echo
echo "Installation started."

echo -e "label: gpt\n- 500MiB U -\n- - L -" | sfdisk $DISK


echo "Waiting for partitions to be online"

while ! [ -e $DISK*1 ]
do
  sleep 5
done

echo "Partitions created."

mkfs -t vfat $DISK*1
mkfs -F -t ext4 $DISK*2

echo "File systems created."

mount $DISK*2 /mnt
mount --mkdir $DISK*1 /mnt/boot

echo "Configuring mirrors"

reflector --score 5 --fastest 5 -c "Switzerland,Italy" --save /etc/pacman.d/mirrorlist


echo "Installing base system"

pacman-key --init
pacman-key --populate
pacstrap -K /mnt base linux linux-firmware

echo "Configuring system"

genfstab -U /mnt >> /mnt/etc/fstab

cp -ra /install /mnt
mkdir -p /mnt/

arch-chroot /mnt bash /install/run_install_scripts.sh 

rm -rf /mnt/install

reboot
