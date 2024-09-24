.PHONY : all build objcopy run debug connect clean

CARGO = cargo
ARCH = loongarch64-unknown-none
BUILD_TYPE = release
CARGO_FLAGS = --$(BUILD_TYPE)
TARGET = kernel

QEMU = qemu-system-loongarch64
MACHINE = virt
QEMU_ARGS = -bios target/$(ARCH)/$(BUILD_TYPE)/$(TARGET).bin \
	-machine $(MACHINE) \
	-nographic

GDB = loongarch64-unknown-linux-gnu-gdb

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

objcopy: build
	@$(CARGO) objcopy $(CARGO_FLAGS) -- -O binary target/$(ARCH)/$(BUILD_TYPE)/$(TARGET).bin

run: objcopy
	@$(QEMU) $(QEMU_ARGS)

debug:
	@$(QEMU) $(QEMU_ARGS) -s -S

connect:
	@$(GDB) \
	-ex "file target/$(ARCH)/$(BUILD_TYPE)/$(TARGET)" \
	-ex "target remote :1234"

clean:
	@$(CARGO) clean