# dpdk-sq

Rust bindings for DPDK (Data Plane Development Kit) - providing safe and idiomatic Rust interfaces to DPDK's high-performance packet processing capabilities.

## Project Structure

- `dpdk-sys/` - Low-level FFI bindings (auto-generated with bindgen)
- `dpdk/` - High-level safe Rust API
- `examples/` - Example applications

## Features

- Type-safe wrapper around DPDK C APIs
- Memory safety guarantees
- Zero-cost abstractions
- Idiomatic Rust error handling
- Safe wrappers for:
  - Environment Abstraction Layer (EAL)
  - Memory pools
  - Packet buffers (mbufs)

## Prerequisites

Before building this project, you need to install DPDK:

### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install dpdk dpdk-dev pkg-config libclang-dev
```

### Fedora/RHEL

```bash
sudo dnf install dpdk dpdk-devel pkg-config clang-devel
```

### From Source

See the [DPDK Getting Started Guide](https://doc.dpdk.org/guides/linux_gsg/build_dpdk.html) for building DPDK from source.

Verify installation:

```bash
pkg-config --modversion libdpdk
```

## Building

Build the workspace:

```bash
cargo build
```

Build with release optimizations:

```bash
cargo build --release
```

## Running Examples

### Basic Initialization Example

This example demonstrates EAL initialization, memory pool creation, and mbuf operations:

```bash
# Note: DPDK applications typically require root privileges or proper capability settings
sudo -E cargo run --example basic-init

# With custom EAL arguments:
sudo -E cargo run --example basic-init -- -c 0x3 -n 4
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dpdk = { path = "path/to/dpdk-sq/dpdk" }
```

Example usage:

```rust
use dpdk::{Eal, Mempool, Mbuf};

fn main() -> dpdk::Result<()> {
    // Initialize DPDK EAL
    let eal = Eal::init(vec![
        "myapp",
        "-c", "0x1",  // Use CPU core 0
        "-n", "4",     // 4 memory channels
    ])?;

    // Create a memory pool for packets
    let mempool = Mempool::create(
        "packet_pool",
        8192,    // Number of elements
        2048,    // Element size
        256,     // Cache size
        -1,      // Any NUMA socket
    )?;

    // Allocate a packet buffer
    let mut mbuf = Mbuf::alloc(&mempool)?;
    
    // Add data to the packet
    let data = mbuf.append(64)?;
    data.copy_from_slice(&[0u8; 64]);

    Ok(())
}
```

## Architecture

### dpdk-sys

Low-level FFI bindings auto-generated from DPDK headers using `bindgen`. This crate provides raw, unsafe access to DPDK functions.

### dpdk

High-level safe Rust API that wraps `dpdk-sys` with:
- RAII-based resource management
- Rust error handling
- Safe abstractions that prevent common mistakes
- Documentation and examples

## Development

Run tests:

```bash
cargo test
```

Note: Integration tests require DPDK to be installed and may need root privileges.

Check formatting:

```bash
cargo fmt --check
```

Run clippy:

```bash
cargo clippy -- -D warnings
```

## Status

🚧 Work in Progress

Implemented:
- [x] Basic EAL initialization
- [x] Memory pool management
- [x] Mbuf allocation and operations
- [x] Error handling

Planned:
- [ ] Ethernet device operations
- [ ] Ring buffers
- [ ] Multi-core launching
- [ ] Flow API
- [ ] Additional protocol headers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

- [DPDK Documentation](https://doc.dpdk.org/)
- [DPDK Programmer's Guide](https://doc.dpdk.org/guides/prog_guide/)
- [DPDK API Reference](https://doc.dpdk.org/api/)
