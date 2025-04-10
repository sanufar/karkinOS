.PHONY: all build qemu clean

all: qemu

qemu: 
	@echo "Booting from QEMU."
	@cargo run --bin qemu-uefi

clean:
	@echo "Cleaning workspace…"
	@cargo clean
	@rm -rf build

