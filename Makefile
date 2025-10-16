.PHONY: help build test check fmt clippy doc clean install-deps

help:
	@echo "Available targets:"
	@echo "  build        - Build all crates"
	@echo "  test         - Run all tests"
	@echo "  check        - Run cargo check"
	@echo "  fmt          - Format code"
	@echo "  clippy       - Run clippy linter"
	@echo "  doc          - Generate documentation"
	@echo "  clean        - Clean build artifacts"
	@echo "  install-deps - Install DPDK and dependencies (Ubuntu/Debian)"

build:
	cargo build --all

test:
	cargo test --all

check:
	cargo check --all

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

doc:
	cargo doc --no-deps --open

clean:
	cargo clean

install-deps:
	@echo "Installing DPDK and dependencies..."
	sudo apt-get update
	sudo apt-get install -y dpdk dpdk-dev pkg-config libclang-dev build-essential
	@echo "Done! Verify with: pkg-config --modversion libdpdk"
