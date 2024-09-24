.PHONY : all build run debug connect clean setup

RUSTUP = rustup
CARGO = cargo
ARCH = loongarch64-unknown-none

KERNEL = kernel

all:
	@cd $(KERNEL) && make -s all

run:
	@cd $(KERNEL) && make -s run

build:
	@cd $(KERNEL) && make -s build

debug:
	@cd $(KERNEL) && make -s debug

connect:
	@cd $(KERNEL) && make -s connect

clean:
	@cd $(KERNEL) && make -s clean

setup:
	@$(RUSTUP) target add $(ARCH)
	@$(RUSTUP) default nightly-2024-07-26
	@$(CARGO) install cargo-binutils
	@$(RUSTUP) component add rust-src
	@$(RUSTUP) component add llvm-tools-preview

%:
	@cd $(KERNEL) && make -s $@