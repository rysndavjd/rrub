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
    mkdir -p ./esp/efi/boot
fi

cargo build --target x86_64-unknown-uefi

cp "./target/x86_64-unknown-uefi/debug/rrub.efi" "./esp/efi/boot/bootx64.efi"

qemu-system-x86_64 \
    -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp

