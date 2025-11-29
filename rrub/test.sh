#!/bin/bash

if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Please install qemu-system-x86_64"
    exit 1
fi

#
#if ! command -v qemu-system-aarch64 &> /dev/null; then
#    echo "Please install qemu-system-aarch64"
#    exit 1
#fi

if [ ! -f "./OVMF_CODE.fd" ] || [ ! -f "./OVMF_VARS.fd" ]; then
    echo "Missing OVMF_CODE.fd and OVMF_VARS.fd"
    exit 1
fi

if [ ! -d "./esp" ]; then
    mkdir -p ./esp/EFI/BOOT
fi

cargo build --target x86_64-unknown-uefi

cp -rf "./target/x86_64-unknown-uefi/debug/rrub.efi" "./esp/EFI/BOOT/BOOTX64.EFI"

qemu-system-x86_64 \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
  -m 8G \
  -drive format=raw,file=fat:rw:esp \
  -device qemu-xhci \
  -device usb-kbd

#   -device virtio-vga,edid=on,xres=1024,yres=768 
