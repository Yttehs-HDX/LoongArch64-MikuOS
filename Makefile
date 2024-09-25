.PHONY : all all-user build build-user run debug connect clean clean-user setup

RUSTUP = rustup
CARGO = cargo
ARCH = loongarch64-unknown-none

KERNEL = kernel
USER = user

all: all-user
	@cd $(KERNEL) && make -s all

all-user:
	@cd $(USER) && make -s all

run:
	@cd $(KERNEL) && make -s run

build: build-user
	@cd $(KERNEL) && make -s build

build-user:
	@cd $(USER) && make -s objcopy

debug:
	@cd $(KERNEL) && make -s debug

connect:
	@cd $(KERNEL) && make -s connect

clean: clean-user
	@cd $(KERNEL) && make -s clean

clean-user:
	@cd $(USER) && make -s clean

setup:
	@$(RUSTUP) target add $(ARCH)
	@$(RUSTUP) default nightly-2024-07-26
	@$(CARGO) install cargo-binutils
	@$(RUSTUP) component add rust-src
	@$(RUSTUP) component add llvm-tools-preview

%:
	@cd $(KERNEL) && make -s $@