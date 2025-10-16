# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with workspace setup
- `dpdk-sys` crate for low-level FFI bindings
  - Auto-generated bindings using bindgen
  - Support for DPDK 20.11+
- `dpdk` crate for high-level safe API
  - EAL (Environment Abstraction Layer) wrapper
  - Mempool management with RAII
  - Mbuf (packet buffer) operations
  - Safe error handling with custom Error types
- Example applications
  - `basic-init`: Demonstrates EAL initialization and basic usage
- Documentation
  - Comprehensive README with installation instructions
  - Per-crate documentation
  - Contributing guidelines
- CI/CD pipeline
  - Format checking
  - Clippy linting
  - Build verification across stable/beta/nightly
  - Documentation generation
- Development tools
  - Makefile for common tasks
  - .gitignore for Rust projects
  - rust-toolchain.toml

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - Unreleased

- Initial release (work in progress)
