#!/bin/bash

mkdir -p ./build

nasm -f bin ./src/stage1.asm -o build/stage1.bin

qemu-system-x86_64 -drive format=raw,file=build/stage1.bin -monitor stdio