
RUSTFLAGS="-C target-cpu=native"


.PHONY: build build-release build-production check test doc doc_book bench rust-update tree clean

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
	@cargo test

doc:
	@cargo doc --no-deps

doc-book:
	@mdbook clean --dest-dir=target/doc_book doc_book
	@mdbook build --dest-dir ../target/doc_book doc_book
	@mdbook test -d ../target/doc_book doc_book #-L

bench:
	@cargo bench

rust-update:
	@rustup update

tree:
	@cargo tree
	@echo
	@echo "Directory tree:"
	@tree -I target

.PHONY: bench-polynomial
bench-polynomial:
	@cargo bench --bench polynomial_benchmark

.PHONY: bench-foo
bench-foo:
	@cargo bench --bench foo_benchmark

.PHONY: run-polynomial
run-polynomial:
	@cargo run --bin rustamath-polynomial

.PHONY: run-lottery
run-lottery:
	@cargo run --bin rustamath-lottery
