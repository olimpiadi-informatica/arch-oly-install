#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="arch-olympiads-install"
iso_label="ARCH_OLY_INSTALL_$(git rev-parse --short HEAD)"
iso_publisher="Olimpiadi Italiane di Informatica <https://olinfo.it>"
iso_application="Arch Linux installer for Olympiads"
iso_version="$(git rev-parse --short HEAD)"
install_dir="arch-oly"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito'
           'uefi-x64.grub.esp' 'uefi-x64.grub.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/setup.sh"]="0:0:755"
)
