//! Error types for DPDK operations

use std::fmt;
use thiserror::Error;

/// Result type for DPDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur when working with DPDK
#[derive(Error, Debug)]
pub enum Error {
    /// EAL initialization failed
    #[error("EAL initialization failed: {0}")]
    EalInit(String),

    /// Memory pool creation or operation failed
    #[error("Mempool operation failed: {0}")]
    Mempool(String),

    /// Mbuf operation failed
    #[error("Mbuf operation failed: {0}")]
    Mbuf(String),

    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// Generic DPDK error with error code
    #[error("DPDK error (code {code}): {message}")]
    Dpdk { code: i32, message: String },
}

impl Error {
    /// Create a new DPDK error from an error code
    pub fn from_errno(code: i32) -> Self {
        let message = match code {
            -1 => "Operation not permitted".to_string(),
            -2 => "No such file or directory".to_string(),
            -12 => "Out of memory".to_string(),
            -22 => "Invalid argument".to_string(),
            -95 => "Operation not supported".to_string(),
            _ => format!("Unknown error code: {}", code),
        };
        Error::Dpdk { code, message }
    }
}
