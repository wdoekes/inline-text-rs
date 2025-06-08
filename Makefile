.PHONY: all debug test
all: debug test

debug:
	cargo build

test:
	cargo test
