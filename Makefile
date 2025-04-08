UNAME := $(shell uname)

# -----------------------------------------------------------------------------
# Tool locations
# -----------------------------------------------------------------------------
ifeq ($(UNAME),Darwin)
	OBJCOPY := $(shell brew --prefix binutils)/bin/objcopy
	QEMU    := qemu-system-x86_64
endif

ifeq ($(UNAME),Linux)
	OBJCOPY := objcopy
	QEMU    := qemu-system-x86_64
endif

CARGO := cargo +nightly
export CARGO_TARGET_DIR := $(CURDIR)/target

# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------
.PHONY: all get-deps build objcopy image run debug clean

all: get-deps build objcopy image
	@echo "Paneros boot_init has been successfully built!"

# -----------------------------------------------------------------------------
# Install/verify build tools (macOS only for now)
# -----------------------------------------------------------------------------
get-deps:
ifeq ($(UNAME),Darwin)
	@echo "Downloading macOS build tools…"
	@brew list binutils >/dev/null || brew install binutils
endif

# -----------------------------------------------------------------------------
# Build the boot sector (debug profile, custom 16‑bit target)
# -----------------------------------------------------------------------------
build:
	@echo "Building Karkinos…"
	@$(CARGO) build --target=x86_16.json --package=boot_init

# -----------------------------------------------------------------------------
# Convert ELF → flat binary
# -----------------------------------------------------------------------------
objcopy:
	@echo "Creating raw boot sector binary…"
	@mkdir -p build
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_16/debug/boot_init build/boot.bin

# -----------------------------------------------------------------------------
# Produce a minimal 10 MiB disk image with the boot sector at LBA 0
# -----------------------------------------------------------------------------
image:
	@echo "Creating disk image…"
	@mkdir -p build
	@dd if=/dev/zero of=build/disk.img bs=1M count=10 status=none
	@dd if=build/boot.bin of=build/disk.img conv=notrunc status=none

# -----------------------------------------------------------------------------
# Run / debug in QEMU
# -----------------------------------------------------------------------------
run: all
	@echo "Running Karkinos…"
	@$(QEMU) -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide

debug: all
	@echo "Debugging Karkinos (QEMU waiting for GDB on port 1234)…"
	@$(QEMU) -S -s -drive file=build/disk.img,index=0,media=disk,format=raw,if=ide

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------
clean:
	@echo "Cleaning workspace…"
	@cargo clean
	@rm -rf build

