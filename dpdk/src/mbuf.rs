//! Packet buffer (mbuf) operations
//!
//! Mbufs are the fundamental data structure for packet processing in DPDK.
//! They represent network packets and associated metadata.

use crate::error::{Error, Result};
use crate::mempool::Mempool;
use dpdk_sys as ffi;
use std::ptr;
use std::slice;

/// Packet buffer (mbuf)
///
/// Represents a network packet with its data and metadata.
pub struct Mbuf {
    raw: *mut ffi::rte_mbuf,
    owned: bool,
}

impl Mbuf {
    /// Allocate a new mbuf from a mempool
    ///
    /// # Arguments
    ///
    /// * `mempool` - The mempool to allocate from
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dpdk::{Mempool, Mbuf};
    ///
    /// let mempool = Mempool::create("packet_pool", 8192, 2048, 256, -1)
    ///     .expect("Failed to create mempool");
    /// let mbuf = Mbuf::alloc(&mempool).expect("Failed to allocate mbuf");
    /// ```
    pub fn alloc(mempool: &Mempool) -> Result<Self> {
        let raw = unsafe { ffi::rte_pktmbuf_alloc(mempool.as_ptr()) };

        if raw.is_null() {
            return Err(Error::Mbuf("Failed to allocate mbuf".to_string()));
        }

        Ok(Mbuf { raw, owned: true })
    }

    /// Create an Mbuf from a raw pointer without taking ownership
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is valid and remains valid
    /// for the lifetime of this Mbuf.
    pub unsafe fn from_raw(raw: *mut ffi::rte_mbuf) -> Self {
        Mbuf { raw, owned: false }
    }

    /// Get a pointer to the raw mbuf structure
    pub fn as_ptr(&self) -> *mut ffi::rte_mbuf {
        self.raw
    }

    /// Get the data length of the mbuf
    pub fn data_len(&self) -> u16 {
        unsafe { (*self.raw).data_len }
    }

    /// Get the packet length (total length for chained mbufs)
    pub fn pkt_len(&self) -> u32 {
        unsafe { (*self.raw).pkt_len }
    }

    /// Get a reference to the packet data
    pub fn data(&self) -> &[u8] {
        unsafe {
            let data_ptr = ffi::rte_pktmbuf_mtod_offset(self.raw, 0) as *const u8;
            slice::from_raw_parts(data_ptr, self.data_len() as usize)
        }
    }

    /// Get a mutable reference to the packet data
    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe {
            let data_ptr = ffi::rte_pktmbuf_mtod_offset(self.raw, 0) as *mut u8;
            slice::from_raw_parts_mut(data_ptr, self.data_len() as usize)
        }
    }

    /// Append data to the mbuf
    ///
    /// Returns a mutable slice to the appended area
    pub fn append(&mut self, len: u16) -> Result<&mut [u8]> {
        let ptr = unsafe { ffi::rte_pktmbuf_append(self.raw, len) };

        if ptr.is_null() {
            return Err(Error::Mbuf("Failed to append data to mbuf".to_string()));
        }

        Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut u8, len as usize) })
    }

    /// Prepend data to the mbuf
    ///
    /// Returns a mutable slice to the prepended area
    pub fn prepend(&mut self, len: u16) -> Result<&mut [u8]> {
        let ptr = unsafe { ffi::rte_pktmbuf_prepend(self.raw, len) };

        if ptr.is_null() {
            return Err(Error::Mbuf("Failed to prepend data to mbuf".to_string()));
        }

        Ok(unsafe { slice::from_raw_parts_mut(ptr as *mut u8, len as usize) })
    }
}

impl Drop for Mbuf {
    fn drop(&mut self) {
        if self.owned && !self.raw.is_null() {
            unsafe {
                ffi::rte_pktmbuf_free(self.raw);
            }
        }
    }
}

// Mbuf is not thread-safe by default (single ownership model)
// Users can use Send if they ensure exclusive access
unsafe impl Send for Mbuf {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbuf_sizes() {
        // These tests don't require DPDK initialization
        // Just checking that the size calculations are sane
        assert!(std::mem::size_of::<Mbuf>() <= 16);
    }
}
