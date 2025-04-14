#!/bin/bash
set -e

TMP_DIR="${TMPDIR:-/tmp}"
TEST_IMG="$TMP_DIR/kernel_test_uefi.img"

if [ -f "$TEST_IMG" ]; then
    echo "Removing old test image: $TEST_IMG"
    rm "$TEST_IMG"
fi

# echo "Cleaning tests directory..."
# cargo clean --target-dir tests

echo "Building kernel tests..."
RUSTFLAGS="-C debuginfo=0" cargo build -p kernel --features kerntest -j 8 --target x86_64-unknown-none --target-dir tests

echo "Locating test binary..."
TEST_BIN=$(rg --files tests/x86_64-unknown-none/debug/deps/ | rg -v '\.' | head -n 1)

if [ -z "$TEST_BIN" ]; then
    echo "Error: Test binary not found!"
    exit 1
fi

echo "Found test binary: $TEST_BIN"
export KERNEL_TEST_BIN="$TEST_BIN"

echo "Launching QEMU with kernel test binary..."
cargo run --bin kernel-test

