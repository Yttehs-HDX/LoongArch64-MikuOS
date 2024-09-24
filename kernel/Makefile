.PHONY : all build objcopy run clean

CARGO = cargo
ARCH = loongarch64-unknown-none
BUILD_TYPE = release
CARGO_FLAGS = --$(BUILD_TYPE)

QEMU = qemu-system-loongarch64
MACHINE = virt

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

objcopy: build
	@$(CARGO) objcopy $(CARGO_FLAGS) -- -O binary target/$(ARCH)/$(BUILD_TYPE)/kernel.bin

run: objcopy
	@$(QEMU) \
	-bios target/$(ARCH)/$(BUILD_TYPE)/kernel.bin \
	-machine $(MACHINE) \
	-nographic \

clean:
	@$(CARGO) clean