#!/bin/bash

cleanup_working_dir() {
    if [[ -d "${working_dir}" ]]; then
        rm -rf -- "${working_dir}"
    fi
}

working_dir="$(mktemp -dt test_iso.XXXXXXXXXX)"
trap cleanup_working_dir EXIT

# Set up UEFI
if [[ ! -f '/usr/share/edk2/x64/OVMF_VARS.4m.fd' ]]; then
    printf 'ERROR: %s\n' "OVMF_VARS.4m.fd not found. Install edk2-ovmf."
    exit 1
fi
cp -a -- '/usr/share/edk2/x64/OVMF_VARS.4m.fd' "${working_dir}/"

dd if=/dev/zero of=${working_dir}/disk.img bs=1M count=0 seek=102400 &> /dev/null

ISO=$(ls out/arch-* | head -n1)
qemu-system-x86_64 -m 4G \
  -drive "if=pflash,format=raw,unit=0,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd,read-only=on" \
  -drive "if=pflash,format=raw,unit=1,file=${working_dir}/OVMF_VARS.4m.fd" \
  -global "driver=cfi.pflash01,property=secure,value=off" \
  -drive file=${working_dir}/disk.img,if=none,id=nvm,format=raw \
  -device nvme,serial=deadbeef,drive=nvm \
  -cdrom $ISO -enable-kvm
