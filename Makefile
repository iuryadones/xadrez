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

web-setup:
	@which trunk > /dev/null 2>&1 || cargo install trunk
	rustup target add wasm32-unknown-unknown

web:
	cd chess-wasm && trunk serve --open

web-build:
	cd chess-wasm && trunk build --release
