build:
	cargo build

run: build
	./target/debug/merkle-tree
