#!/bin/bash

QEMU=/Users/robey/code/qemu/arm-softmmu/qemu-system-arm

$QEMU -m 256 -M raspi2 -serial stdio -kernel target/kernel/myos.elf
