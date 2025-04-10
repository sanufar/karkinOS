UNAME := $(shell uname)

# -----------------------------------------------------------------------------
# Tool locations
# -----------------------------------------------------------------------------
ifeq ($(UNAME),Darwin)
	OBJCOPY := $(shell brew --prefix binutils)/bin/objcopy
	QEMU    := qemu-system-x86_64
	SFDISK := $(shell brew --prefix util-linux)/sbin/sfdisk
endif

ifeq ($(UNAME),Linux)
	OBJCOPY := objcopy
	QEMU    := qemu-system-x86_64
	SFDISK := /sbin/sfdisk
endif

CARGO := cargo +nightly
export CARGO_TARGET_DIR := $(CURDIR)/target

DISK_LAYOUT = disk.layout
DISK_IMG = build/disk.img


# -----------------------------------------------------------------------------
# Phony targets
# -----------------------------------------------------------------------------
.PHONY: all get-deps build objcopy image run debug clean

all: get-deps build objcopy image
	@echo "Karkinos boot_init has been successfully built!"

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
	@$(CARGO) build --target=x86_16.json --package=bootloader
	@$(CARGO) build --target=x86_64.json --package=kernel

# -----------------------------------------------------------------------------
# Convert ELF → flat binary
# -----------------------------------------------------------------------------
objcopy:
	@echo "Creating raw boot sector binary…"
	@mkdir -p build
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_16/debug/boot_init build/boot_init.bin
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_16/debug/bootloader build/bootloader.bin
	@$(OBJCOPY) -I elf32-i386 -O binary target/x86_64/debug/kernel build/kernel.bin


# -----------------------------------------------------------------------------
# Produce a minimal 10 MiB disk image with the boot sector at LBA 0
# -----------------------------------------------------------------------------
image:
	@echo "Creating disk image…"
	@mkdir -p build
	@dd if=/dev/zero of=$(DISK_IMG) bs=1M count=100 status=none

	@echo "Partitioning disk image..."
	$(SFDISK) build/disk.img < disk.layout

	@echo "Writing boot sector (MBR)..."
	dd if=build/boot_init.bin of=build/disk.img bs=512 count=1 conv=notrunc

	@echo "Writing bootloader to partition (sector 2048)..."
	dd if=build/bootloader.bin of=build/disk.img bs=512 seek=2048 conv=notrunc

	@echo "Writing kernel to partition (sector 4096)..."
	dd if=build/kernel.bin of=build/disk.img bs=512 seek=4096 conv=notrunc

	@echo "Disk image build complete."

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

