# dpdk

Safe and idiomatic Rust bindings to DPDK (Data Plane Development Kit).

This crate provides high-level, safe wrappers around DPDK, enabling high-performance packet processing in Rust with memory safety guarantees.

## Features

- **Type Safety**: Strong typing prevents common C programming errors
- **Memory Safety**: RAII-based resource management ensures proper cleanup
- **Zero-Cost Abstractions**: Minimal overhead over raw DPDK calls
- **Idiomatic Rust**: Error handling with `Result`, iterators, and Rust patterns

## Modules

- **eal**: Environment Abstraction Layer - Initialize DPDK runtime
- **mempool**: Memory pool management for efficient object allocation
- **mbuf**: Packet buffer operations with safe abstractions
- **error**: Error types and handling

## Quick Start

```rust
use dpdk::{Eal, Mempool, Mbuf};

fn main() -> dpdk::Result<()> {
    // Initialize DPDK
    let eal = Eal::init(vec![
        "myapp",
        "-c", "0x1",  // Use CPU core 0
        "-n", "4",     // 4 memory channels
    ])?;

    // Create memory pool
    let mempool = Mempool::create(
        "packet_pool",
        8192,    // Number of elements
        2048,    // Element size
        256,     // Cache size
        -1,      // Any NUMA socket
    )?;

    // Allocate packet buffer
    let mut mbuf = Mbuf::alloc(&mempool)?;
    
    // Write data to packet
    let data = mbuf.append(64)?;
    data.copy_from_slice(&[0u8; 64]);

    println!("Packet length: {}", mbuf.pkt_len());

    Ok(())
}
```

## Prerequisites

DPDK must be installed on your system. See the main README for installation instructions.

## Safety

This crate provides safe abstractions over unsafe DPDK APIs. However, some operations still require care:

- DPDK must be properly initialized before use
- Mbufs must not outlive their mempool
- Multi-threaded access requires proper synchronization

## Examples

See the `examples/` directory for complete working examples.

## Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## Resources

- [DPDK Documentation](https://doc.dpdk.org/)
- [DPDK Programmer's Guide](https://doc.dpdk.org/guides/prog_guide/)
