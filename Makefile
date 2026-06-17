.PHONY: setup build clean test run fmt lint lint-wasm check quality web-setup web-run web-build web-deploy

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

lint-wasm:
	cd chess-wasm && cargo clippy -- -D warnings

check:
	cargo check

quality: lint lint-wasm test
	@echo "✓ Qualidade verificada: lint, wasm-lint, testes"

bench:
	cargo bench

bench-dev:
	CARGO_PROFILE_BENCH_OPT_LEVEL=1 cargo bench

bench-baseline:
	cargo bench --bench bench_space --bench bench_energy --bench bench_time -- --save-baseline baseline

bench-opt:
	cargo bench --bench bench_space --bench bench_energy --bench bench_time -- --save-baseline optimized

bench-all: bench-baseline bench-opt bench-dev
	@echo "Baselines salvas para release e dev"

compare:
	cargo bench --bench bench_space --bench bench_energy --bench bench_time -- --baseline baseline

triangle:
	cargo run --manifest-path scripts/Cargo.toml --

triangle-release:
	cargo run --manifest-path scripts/Cargo.toml -- --mode release

triangle-dev:
	cargo run --manifest-path scripts/Cargo.toml -- --mode dev

triangle-baseline:
	cargo run --manifest-path scripts/Cargo.toml -- --baseline baseline

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
