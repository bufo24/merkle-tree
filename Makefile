prepare:
	rustup target add wasm32-unknown-unknown

build:
	cargo build

run: build
	./target/debug/merkle-tree

test: build
	cargo test

check-lint:
	cargo clippy -- -D warnings