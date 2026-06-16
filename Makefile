.PHONY: setup build clean test run fmt lint check web-setup web-run web-build web-deploy

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

web-run:
	cd chess-wasm && trunk serve --open

web-build:
	cd chess-wasm && trunk build --release

web-deploy:
	cd chess-wasm && trunk build --release --public-url ./ --dist /tmp/gh-dist
	git checkout gh-pages
	cp /tmp/gh-dist/* .
	git add .
	git commit -m "deploy: atualizar site WASM"
	git push
	git checkout main
