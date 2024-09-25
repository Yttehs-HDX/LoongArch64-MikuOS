.PHONY : all build objcopy run debug connect clean

CARGO = cargo
ARCH = loongarch64-unknown-none
BUILD_TYPE = release
CARGO_FLAGS = --$(BUILD_TYPE)
TARGET = kernel

QEMU = qemu-system-loongarch64
MACHINE = virt
QEMU_ARGS = -kernel target/$(ARCH)/$(BUILD_TYPE)/$(TARGET) \
	-machine $(MACHINE) \
	-nographic

GDB = loongarch64-unknown-linux-gnu-gdb

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

run: build
	@$(QEMU) $(QEMU_ARGS)

debug:
	@$(QEMU) $(QEMU_ARGS) -s -S

connect:
	@$(GDB) \
	-ex "file target/$(ARCH)/$(BUILD_TYPE)/$(TARGET)" \
	-ex "target remote :1234"

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)
