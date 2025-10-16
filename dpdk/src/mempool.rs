//! Memory pool management for packet buffers
//!
//! Memory pools are used to allocate fixed-size objects efficiently.
//! In DPDK, they are primarily used for packet buffers (mbufs).

use crate::error::{Error, Result};
use dpdk_sys as ffi;
use std::ffi::CString;
use std::ptr;

/// Memory pool for allocating objects
///
/// A mempool is a fixed-size pool of objects that can be allocated and freed efficiently.
pub struct Mempool {
    raw: *mut ffi::rte_mempool,
}

impl Mempool {
    /// Create a new memory pool
    ///
    /// # Arguments
    ///
    /// * `name` - Unique name for the mempool
    /// * `n` - Number of elements in the pool
    /// * `elt_size` - Size of each element
    /// * `cache_size` - Size of the per-core object cache
    /// * `socket_id` - NUMA socket ID, or -1 for any socket
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dpdk::Mempool;
    ///
    /// let mempool = Mempool::create(
    ///     "packet_pool",
    ///     8192,    // 8K elements
    ///     2048,    // 2KB per element
    ///     256,     // cache size
    ///     -1,      // any socket
    /// ).expect("Failed to create mempool");
    /// ```
    pub fn create(
        name: &str,
        n: u32,
        elt_size: u32,
        cache_size: u32,
        socket_id: i32,
    ) -> Result<Self> {
        let c_name = CString::new(name)
            .map_err(|_| Error::InvalidArgument("Name contains null byte".to_string()))?;

        let raw = unsafe {
            ffi::rte_mempool_create(
                c_name.as_ptr(),
                n,
                elt_size,
                cache_size,
                0, // private data size
                None,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                socket_id,
                0, // flags
            )
        };

        if raw.is_null() {
            return Err(Error::Mempool("Failed to create mempool".to_string()));
        }

        Ok(Mempool { raw })
    }

    /// Get a pointer to the raw mempool structure
    pub fn as_ptr(&self) -> *mut ffi::rte_mempool {
        self.raw
    }

    /// Get the number of available objects in the mempool
    pub fn avail_count(&self) -> u32 {
        unsafe { ffi::rte_mempool_avail_count(self.raw) }
    }

    /// Get the number of objects currently in use
    pub fn in_use_count(&self) -> u32 {
        unsafe { ffi::rte_mempool_in_use_count(self.raw) }
    }
}

impl Drop for Mempool {
    fn drop(&mut self) {
        unsafe {
            ffi::rte_mempool_free(self.raw);
        }
    }
}

// Mempool is thread-safe
unsafe impl Send for Mempool {}
unsafe impl Sync for Mempool {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mempool_name_validation() {
        let result = Mempool::create("test\0pool", 128, 64, 32, -1);
        assert!(result.is_err());
    }
}
