.PHONY: all build test lint ci clean run dev help install-tools

# Default target - what developers should run before committing
all: lint test build

# CI target - exactly what GitHub Actions will run
ci: lint test build

# Full build (all platforms we can build on current OS)
build: build-debug build-release

build-debug:
	@echo "Building debug..."
	cargo build --workspace

build-release:
	@echo "Building release..."
	cargo build --workspace --release

# Run the desktop app
run:
	cargo run --release -p zimler-desktop

dev:
	cargo run -p zimler-desktop

# Test everything
test:
	@echo "Running tests..."
	cargo test --workspace --all-features

test-verbose:
	cargo test --workspace --all-features -- --nocapture

# CRITICAL: Lint checks that MUST pass
lint: fmt-check clippy check

fmt-check:
	@echo "Checking formatting..."
	cargo fmt --all -- --check

clippy:
	@echo "Running clippy..."
	cargo clippy --workspace --all-targets --all-features -- \
		-D warnings \
		-D clippy::all \
		-D clippy::pedantic \
		-D clippy::nursery \
		-D clippy::cargo \
		-W clippy::module_name_repetitions \
		-W clippy::must_use_candidate \
		-W clippy::missing_errors_doc \
		-W clippy::missing_panics_doc

check:
	@echo "Type checking..."
	cargo check --workspace --all-features

# Auto-fix what we can
fix:
	cargo fmt --all
	cargo clippy --workspace --fix --allow-staged --allow-dirty
	cargo fix --workspace --allow-staged --allow-dirty

# Clean everything
clean:
	cargo clean
	rm -rf target/
	rm -rf ios/build/
	rm -rf ios/DerivedData/

# Install required tools
install-tools:
	@echo "Installing required Rust tools..."
	rustup component add clippy
	rustup component add rustfmt
	@echo "Tools installed!"

# Setup git hooks for local development
install-hooks:
	@echo "Setting up git hooks..."
	@echo '#!/bin/sh' > .git/hooks/pre-commit
	@echo 'make lint' >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "Pre-commit hook installed!"

# Documentation
docs:
	cargo doc --workspace --no-deps --open

docs-all:
	cargo doc --workspace --open

# Benchmarks (when we add them)
bench:
	cargo bench --workspace

# Platform-specific builds
build-desktop:
	cargo build --release -p zimler-desktop

# iOS build (when ready)
build-ios:
	@echo "iOS build not yet implemented"
	# cargo lipo --release -p zimler-ffi
	# cd ios && xcodebuild -scheme ZimlerAU -configuration Release

# WASM build (future)
build-wasm:
	@echo "WASM build not yet implemented"
	# wasm-pack build --target web desktop/zimler-desktop

# Help target
help:
	@echo "Zimler Build System"
	@echo ""
	@echo "CRITICAL TARGETS (use these!):"
	@echo "  make         - Run lint, test, and build (do this before committing!)"
	@echo "  make ci      - Exactly what GitHub Actions runs"
	@echo "  make lint    - Run all linting checks (MUST PASS)"
	@echo "  make test    - Run all tests"
	@echo ""
	@echo "DEVELOPMENT:"
	@echo "  make dev     - Run desktop app in debug mode"
	@echo "  make run     - Run desktop app in release mode"
	@echo "  make fix     - Auto-fix formatting and some clippy warnings"
	@echo ""
	@echo "SETUP:"
	@echo "  make install-tools - Install required Rust tools"
	@echo "  make install-hooks - Setup git pre-commit hooks"
	@echo ""
	@echo "OTHER:"
	@echo "  make clean   - Remove all build artifacts"
	@echo "  make docs    - Generate and open documentation"
	@echo "  make bench   - Run benchmarks"

.DEFAULT_GOAL := help