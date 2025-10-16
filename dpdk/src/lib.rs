//! Safe and idiomatic Rust bindings to DPDK (Data Plane Development Kit)
//!
//! This crate provides high-level, safe wrappers around the DPDK C library,
//! enabling high-performance packet processing in Rust with memory safety guarantees.
//!
//! # Modules
//!
//! - [`eal`] - Environment Abstraction Layer initialization and management
//! - [`mempool`] - Memory pool management for packet buffers
//! - [`mbuf`] - Packet buffer (mbuf) operations
//! - [`error`] - Error types and handling

pub mod eal;
pub mod error;
pub mod mbuf;
pub mod mempool;

pub use error::{Error, Result};

// Re-export commonly used types
pub use eal::Eal;
pub use mbuf::Mbuf;
pub use mempool::Mempool;
