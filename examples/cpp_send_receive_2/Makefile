.PHONY: all clean

all: build/Makefile
	@$(MAKE) -C build

build/Makefile:
	@mkdir -p build
	@cd build && cmake ..

clean:
	@$(MAKE) -C build clean
	@rm -rf build

