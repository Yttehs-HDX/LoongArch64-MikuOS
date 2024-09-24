.PHONY : all build run clean

KERNEL = kernel

all:
	@cd $(KERNEL) && make -s all

run:
	@cd $(KERNEL) && make -s run

build:
	@cd $(KERNEL) && make -s build

clean:
	@cd $(KERNEL) && make -s clean

%:
	@cd $(KERNEL) && make -s $@