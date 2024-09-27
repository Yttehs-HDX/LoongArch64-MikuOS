.PHONY : all build run debug connect clean

ARCH := loongarch64-unknown-none
BUILD_TYPE := release
KERNEL := kernel
TARGET := target/$(ARCH)/$(BUILD_TYPE)/$(KERNEL)

CARGO := cargo
CARGO_FLAGS := --$(BUILD_TYPE)

QEMU := qemu-system-loongarch64
QEMU_FLAGS := -machine virt \
	-kernel $(TARGET) \
	-nographic

GDB := rust-gdb
GDB_FLAGS := -ex "file $(TARGET)" \
	-ex "target remote :1234" \
	-ex "set arch loongarch:la64" \
	-ex "layout src" \

all: run

build:
	@$(CARGO) build $(CARGO_FLAGS)

run: build
	@$(QEMU) $(QEMU_FLAGS)

debug:
	@echo -e "\033[33m[+] Run 'make connect' in another terminal at the same dir\033[0m"
	@$(QEMU) $(QEMU_FLAGS) -S -s

connect:
	@$(GDB) $(GDB_FLAGS)

clean:
	@$(CARGO) clean

%:
	@$(CARGO) $@ $(CARGO_FLAGS)