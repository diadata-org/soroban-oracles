.SILENT:
.PHONY: build fmt clean test all

build:
	soroban contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

test: build
	cargo test

all: test

.DEFAULT_GOAL := all
