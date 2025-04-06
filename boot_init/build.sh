# cargo +nightly build --release --target x86_16.json
#
# objcopy -O binary /Users/sanufar/projects/paneros/src/boot_init/target/x86_16/release/boot_init boot.bin
#
# dd if=/dev/zero of=disk.img bs=1M count=10 
#
# dd if=boot.bin of=disk.img bs=512 count=1 conv=notrunc
#
# qemu-system-i386 -drive format=raw,file=disk.img
#

#!/usr/bin/env bash
set -euo pipefail

TARGET=x86_16
export CARGO_TARGET_DIR="${PWD}/target"   # crate‑local target dir

cargo +nightly build --release --target "${TARGET}.json"

BOOT_ELF="${CARGO_TARGET_DIR}/${TARGET}/release/boot_init"
objcopy --binary-architecture i8086 -O binary "$BOOT_ELF" boot.bin

dd if=/dev/zero of=disk.img bs=1M count=10
dd if=boot.bin  of=disk.img bs=512 count=1 conv=notrunc

qemu-system-i386 -drive format=raw,file=disk.img

