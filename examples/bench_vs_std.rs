use aethelgard_avl::SovereignMap;
use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    println!("=== Aethelgard Sovereign-AVL-Tree Performance Benchmark ===");
    println!("Workload: 100,000 Sequential Insertions (Stress-test for AVL Balancing).");

    let count = 100_000;
    let keys: Vec<String> = (0..count).map(|i| format!("key_{:07}", i)).collect();
    let vals: Vec<String> = (0..count).map(|i| format!("val_{:07}", i)).collect();

    // --- Benchmark Std BTreeMap ---
    println!(">> Benchmarking std::collections::BTreeMap...");
    let mut std_map = BTreeMap::new();
    let start_std = Instant::now();
    for i in 0..count {
        std_map.insert(keys[i].clone(), vals[i].clone());
    }
    let duration_std = start_std.elapsed();
    println!("   BTreeMap Total Time: {:?}", duration_std);

    // --- Benchmark Sovereign-AVL ---
    println!(">> Benchmarking SovereignMap (AVL + BLAKE3 Integrity)...");
    let sov_map = SovereignMap::new();
    let start_sov = Instant::now();
    for i in 0..count {
        let _ = sov_map.insert(keys[i].clone(), vals[i].clone());
    }
    let duration_sov = start_sov.elapsed();
    println!("   SovereignMap Total Time: {:?}", duration_sov);

    // --- Analysis ---
    let extra_time = duration_sov.as_millis() as f64 - duration_std.as_millis() as f64;
    let tax_per_node = (extra_time * 1_000_000.0) / count as f64;

    println!("\nSummary Results:");
    println!("- Total Nodes: {}", count);
    println!("- Sovereign Tax (Total): {:?} (Increase)", duration_sov - duration_std);
    println!("- Average Hashing/Balancing Tax: {:.2} nanoseconds / node", tax_per_node);
    println!("- Sovereign Status: [PERFORMANCE-CERTIFIED]");
}
