.PHONY: build test fmt clean all
.DEFAULT_GOAL: build

all: test

test: build
	cargo test

build:
	soroban contract build
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/dia_soroban_random_oracle.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
