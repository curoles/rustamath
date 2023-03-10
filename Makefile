.PHONY: build build-release build-production check test doc bench rust-update tree clean

# Default target
build:
	@cargo build
	@cargo clippy

check:
	@cargo check
	@cargo clippy

clean:
	@cargo clean

build-release:
	@cargo build --release

build-production:
	@cargo build --profile production

test:
	@cargo test #-j 8

doc:
	@cargo doc --no-deps

bench:
	@cargo bench

rust-update:
	@rustup update

tree:
	@cargo tree
	@echo
	@echo "Directory tree:"
	@tree -I target
