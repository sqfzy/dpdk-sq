//! Environment Abstraction Layer (EAL) initialization and management
//!
//! The EAL provides a generic interface to low-level resources such as hardware,
//! memory, and CPU cores. It is the first thing that must be initialized in a DPDK application.

use crate::error::{Error, Result};
use dpdk_sys as ffi;
use std::ffi::{CStr, CString};
use std::ptr;

/// Environment Abstraction Layer handle
///
/// This represents an initialized DPDK EAL instance. When dropped, it will
/// clean up DPDK resources.
pub struct Eal {
    _private: (),
}

impl Eal {
    /// Initialize the Environment Abstraction Layer
    ///
    /// # Arguments
    ///
    /// * `args` - Command line arguments to pass to EAL (e.g., `vec!["-c", "0x1", "-n", "4"]`)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dpdk::Eal;
    ///
    /// let eal = Eal::init(vec![
    ///     "app",
    ///     "-c", "0x1",  // Use core 0
    ///     "-n", "4",     // 4 memory channels
    /// ]).expect("Failed to initialize EAL");
    /// ```
    pub fn init<S: AsRef<str>>(args: Vec<S>) -> Result<Self> {
        // Convert Rust strings to C strings
        let c_args: Result<Vec<CString>> = args
            .iter()
            .map(|s| {
                CString::new(s.as_ref())
                    .map_err(|_| Error::InvalidArgument("Argument contains null byte".to_string()))
            })
            .collect();
        let c_args = c_args?;

        // Create array of pointers
        let mut arg_ptrs: Vec<*mut i8> = c_args
            .iter()
            .map(|s| s.as_ptr() as *mut i8)
            .collect();

        let argc = arg_ptrs.len() as i32;

        // Call rte_eal_init
        let ret = unsafe { ffi::rte_eal_init(argc, arg_ptrs.as_mut_ptr()) };

        if ret < 0 {
            return Err(Error::EalInit(format!(
                "rte_eal_init failed with code {}",
                ret
            )));
        }

        Ok(Eal { _private: () })
    }

    /// Get the number of logical cores
    pub fn lcore_count(&self) -> u32 {
        unsafe { ffi::rte_lcore_count() }
    }

    /// Get the ID of the current logical core
    pub fn lcore_id(&self) -> u32 {
        unsafe { ffi::rte_lcore_id() }
    }

    /// Get the ID of the main logical core
    pub fn get_main_lcore(&self) -> u32 {
        unsafe { ffi::rte_get_main_lcore() }
    }
}

impl Drop for Eal {
    fn drop(&mut self) {
        unsafe {
            ffi::rte_eal_cleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eal_init_validates_args() {
        // Test that we handle null bytes in arguments
        let result = Eal::init(vec!["test\0app"]);
        assert!(result.is_err());
    }
}
