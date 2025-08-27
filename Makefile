.PHONY: all build run clean test desktop

all: desktop

desktop:
	cargo build --release -p zimler-desktop

run:
	cargo run --release -p zimler-desktop

dev:
	cargo run -p zimler-desktop

test:
	cargo test --workspace

clean:
	cargo clean

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings