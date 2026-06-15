.PHONY: setup build clean test run fmt lint check

setup:
	@which rustup > /dev/null 2>&1 || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup update

build:
	cargo build

clean:
	cargo clean

test:
	cargo test -- --nocapture

run:
	cargo run

fmt:
	cargo fmt --all

lint:
	cargo clippy -- -D warnings

check:
	cargo check
