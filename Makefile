.PHONY: build test doc bench

# Default target
build:
	cargo build

test:
	cargo test

doc:
	cargo doc

bench:
	cargo bench
