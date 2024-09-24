.PHONY : all build run clean

KERNEL = kernel

all:
	@cd $(KERNEL) && make -s all

run:
	@cd $(KERNEL) && make -s run

build:
	@cd $(KERNEL) && make -s objcopy

debug:
	@cd $(KERNEL) && make -s debug

connect:
	@cd $(KERNEL) && make -s connect

clean:
	@cd $(KERNEL) && make -s clean

%:
	@cd $(KERNEL) && make -s $@