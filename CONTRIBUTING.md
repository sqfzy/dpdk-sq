# Contributing to dpdk-sq

Thank you for your interest in contributing to dpdk-sq! This document provides guidelines and instructions for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/dpdk-sq.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature-name`
8. Submit a pull request

## Development Setup

### Prerequisites

- Rust (stable, beta, or nightly)
- DPDK library installed (see README.md for installation instructions)
- pkg-config
- libclang (for bindgen)

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Formatting

Before submitting a PR, ensure your code is properly formatted:

```bash
cargo fmt
```

### Linting

Run clippy to catch common mistakes:

```bash
cargo clippy -- -D warnings
```

## Code Style

- Follow the standard Rust style guide
- Use `cargo fmt` to format code
- Write documentation for public APIs
- Add tests for new functionality
- Keep commits atomic and well-described

## Pull Request Guidelines

1. **Update Documentation**: If you change APIs, update the documentation
2. **Add Tests**: Include tests for new features or bug fixes
3. **Follow Code Style**: Use `cargo fmt` and `cargo clippy`
4. **Write Clear Commit Messages**: Describe what and why, not how
5. **Keep PRs Focused**: One feature/fix per PR

## Commit Message Format

We follow conventional commits:

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat: add ethernet device wrapper

Implement safe Rust wrapper around rte_ethdev APIs,
including device configuration and packet RX/TX.
```

## Testing Guidelines

- Write unit tests for individual functions
- Write integration tests for API workflows
- Document test requirements (e.g., needs DPDK installed)
- Use descriptive test names

## Documentation

- All public APIs must have doc comments
- Include examples in doc comments where appropriate
- Update README.md for significant changes
- Keep documentation up-to-date with code changes

## Questions?

If you have questions, feel free to:
- Open an issue for discussion
- Ask in the pull request
- Contact the maintainers

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
