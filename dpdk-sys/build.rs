use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // Try to use pkg-config to find DPDK
    let dpdk = pkg_config::Config::new()
        .atleast_version("20.11")
        .probe("libdpdk");

    match dpdk {
        Ok(library) => {
            // DPDK found via pkg-config
            for include_path in &library.include_paths {
                println!("cargo:include={}", include_path.display());
            }
        }
        Err(_) => {
            // Fallback: assume DPDK is installed in standard locations
            println!("cargo:warning=DPDK not found via pkg-config, using fallback");
            println!("cargo:warning=Make sure DPDK is installed (pkg-config --modversion libdpdk)");

            // Try common installation paths
            let common_paths = vec![
                "/usr/local/include/dpdk",
                "/usr/include/dpdk",
                "/opt/dpdk/include",
            ];

            for path in &common_paths {
                println!("cargo:include={}", path);
            }
        }
    }

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Allowlist for DPDK functions
        .allowlist_function("rte_.*")
        .allowlist_type("rte_.*")
        .allowlist_var("RTE_.*")
        // Common DPDK types
        .allowlist_type("ether_addr")
        // Generate proper types
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        .impl_debug(true)
        // Use core instead of std for no_std compatibility (optional)
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
