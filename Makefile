.PHONY: build test clean

build:
	cargo build --release

test: build
	./test.sh

clean:
	cargo clean