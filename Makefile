.PHONY: build test clean

build:
	cargo build

test: build
	./test.sh

clean:
	cargo clean