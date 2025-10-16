//! Basic DPDK Initialization Example
//!
//! This example demonstrates how to:
//! - Initialize the DPDK Environment Abstraction Layer (EAL)
//! - Create a memory pool
//! - Allocate and manipulate packet buffers (mbufs)
//! - Query system information

use dpdk::{Eal, Mbuf, Mempool};
use std::env;

fn main() {
    println!("=== DPDK Basic Initialization Example ===\n");

    // Get program arguments for EAL
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, use defaults
    let eal_args = if args.len() > 1 {
        args
    } else {
        vec![
            "basic-init".to_string(),
            "-c".to_string(),
            "0x1".to_string(), // Use core 0
            "-n".to_string(),
            "4".to_string(), // 4 memory channels
        ]
    };

    println!("Initializing EAL with args: {:?}", eal_args);

    // Initialize DPDK EAL
    let eal = match Eal::init(eal_args) {
        Ok(eal) => {
            println!("✓ EAL initialized successfully");
            eal
        }
        Err(e) => {
            eprintln!("✗ Failed to initialize EAL: {}", e);
            eprintln!("\nNote: This example requires DPDK to be installed.");
            eprintln!("Install DPDK with: sudo apt-get install dpdk dpdk-dev (Ubuntu/Debian)");
            eprintln!(
                "Or build from source: https://doc.dpdk.org/guides/linux_gsg/build_dpdk.html"
            );
            std::process::exit(1);
        }
    };

    // Query EAL information
    println!("\n=== System Information ===");
    println!("Number of lcores: {}", eal.lcore_count());
    println!("Main lcore ID: {}", eal.get_main_lcore());
    println!("Current lcore ID: {}", eal.lcore_id());

    // Create a memory pool for packet buffers
    println!("\n=== Creating Memory Pool ===");
    let mempool = match Mempool::create(
        "packet_pool",
        8192, // Number of elements
        2048, // Element size (enough for standard packets)
        256,  // Cache size
        -1,   // Socket ID (-1 = any socket)
    ) {
        Ok(pool) => {
            println!("✓ Memory pool 'packet_pool' created successfully");
            pool
        }
        Err(e) => {
            eprintln!("✗ Failed to create memory pool: {}", e);
            std::process::exit(1);
        }
    };

    println!("Available objects in pool: {}", mempool.avail_count());
    println!("Objects in use: {}", mempool.in_use_count());

    // Allocate some mbufs
    println!("\n=== Allocating Packet Buffers ===");
    let mut mbufs = Vec::new();

    for i in 0..5 {
        match Mbuf::alloc(&mempool) {
            Ok(mbuf) => {
                println!(
                    "✓ Allocated mbuf #{}: data_len={}, pkt_len={}",
                    i + 1,
                    mbuf.data_len(),
                    mbuf.pkt_len()
                );
                mbufs.push(mbuf);
            }
            Err(e) => {
                eprintln!("✗ Failed to allocate mbuf: {}", e);
                break;
            }
        }
    }

    println!(
        "\nObjects in use after allocation: {}",
        mempool.in_use_count()
    );

    // Demonstrate mbuf operations
    if let Some(mut mbuf) = mbufs.first_mut() {
        println!("\n=== Mbuf Operations ===");

        // Append data
        match mbuf.append(64) {
            Ok(data) => {
                println!("✓ Appended 64 bytes to mbuf");
                // Write some data
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte = (i % 256) as u8;
                }
                println!("  New data_len: {}", mbuf.data_len());
            }
            Err(e) => {
                eprintln!("✗ Failed to append data: {}", e);
            }
        }

        // Read data back
        let data = mbuf.data();
        println!("  First 10 bytes: {:?}", &data[..10.min(data.len())]);
    }

    // Cleanup happens automatically when variables go out of scope
    println!("\n=== Cleanup ===");
    drop(mbufs);
    println!(
        "Objects in use after freeing mbufs: {}",
        mempool.in_use_count()
    );

    println!("\n✓ Example completed successfully");
    println!("  EAL and mempool will be cleaned up automatically");
}
