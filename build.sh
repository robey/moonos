#!/bin/bash

STYLE=release
if test x"$1" = x"debug"; then
  STYLE=debug
fi

# embedded rust work on the raspi requires:
#   - rustup (nvm/rvm for rust)
#   - the nightly toolchain (rust stable is too old)
#   - the raspi target (arm-unknown-linux-gnueabihf) (hf = hardware fpu)
#   - the rust source code, for xargo
#   - xargo, to build libcore (rust equivalent of libgcc) for the raspi platform

if which -s rustup; then
  :
else
  echo --- installing rustup
  curl https://sh.rustup.rs -sSf | sh
  source $HOME/.cargo/env
fi

if rustup toolchain list | grep -q nightly; then
  :
else
  echo --- installing nightly rust
  rustup toolchain install nightly
fi

if rustup target list | grep arm-unknown-linux-gnueabihf | grep -q installed; then
  :
else
  echo --- installing arm-unknown-linux-gnueabihf
  rustup target install arm-unknown-linux-gnueabihf
fi

if rustup component list | grep rust-src | grep -q installed; then
  :
else
  echo --- installing rust-src
  rustup component add rust-src
fi

if cargo install --list | grep -q xargo; then
  :
else
  echo --- installing xargo
  mv .cargo/config .cargo/not-config
  cargo install xargo
  mv .cargo/not-config .cargo/config
fi

set -eux
if test $STYLE = release; then
  env RUSTFLAGS="--emit asm" xargo build --release
else
  env RUSTFLAGS="--emit asm" xargo build
fi

# make bootable
rm -rf target/kernel && mkdir -p target/kernel
arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c kernel/boot.S -o target/kernel/boot.o
if test $STYLE = release; then
  arm-none-eabi-gcc -mfloat-abi=hard -n -T kernel/linker.ld -o target/kernel/myos.elf -ffreestanding -O2 -nostdlib -Wl,--gc-sections target/kernel/boot.o target/armv7-unknown-linux-gnueabihf/release/libmoon.a
else
  arm-none-eabi-gcc -mfloat-abi=hard -n -T kernel/linker.ld -o target/kernel/myos.elf -ffreestanding -O2 -nostdlib -Wl,--gc-sections target/kernel/boot.o target/armv7-unknown-linux-gnueabihf/debug/libmoon.a
fi

size -A -x target/kernel/myos.elf

qemu-system-arm -m 256 -M raspi2 -serial stdio -kernel target/kernel/myos.elf
