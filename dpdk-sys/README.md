# dpdk-sys

Low-level FFI (Foreign Function Interface) bindings to DPDK (Data Plane Development Kit).

This crate provides raw, unsafe Rust bindings to DPDK C APIs, auto-generated using `bindgen`.

## Usage

This crate is typically not used directly. Instead, use the high-level `dpdk` crate which provides safe wrappers.

If you need direct access to DPDK C APIs:

```rust
use dpdk_sys as ffi;

unsafe {
    // Call DPDK functions directly
    let ret = ffi::rte_eal_init(argc, argv);
}
```

## Building

This crate requires:
- DPDK library installed on your system
- pkg-config to locate DPDK
- libclang for bindgen

The build script will automatically generate Rust bindings from DPDK headers.

## Supported DPDK Versions

- DPDK 20.11 or later

## Note

All functions in this crate are `unsafe` because they directly call C functions. Use the high-level `dpdk` crate for safe abstractions.
