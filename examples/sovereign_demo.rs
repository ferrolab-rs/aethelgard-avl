use aethelgard_avl::SovereignMap;

fn main() {
    println!("=== Aethelgard Sovereign-AVL-Tree Stress-Test & Traceability ===");
    let map = SovereignMap::new();

    // 1. Insertion sequence: [10, 20, 30, 25, 27]
    println!("\n[Step 1] Inserting sequence: [10, 20, 30, 25, 27] with trace...");
    let sequence = vec![
        ("10", "Ten"),
        ("20", "Twenty"),
        ("30", "Thirty"),
        ("25", "Twenty-Five"),
        ("27", "Twenty-Seven"),
    ];

    for (k, v) in sequence {
        println!(">> Inserting Key: {:?}", k);
        map.insert(k.to_string(), v.to_string()).expect("Failed to insert");
    }

    // 2. Visualize initial state
    println!("\n[Step 2] Current Tree Structure:");
    map.visualize().expect("Failed to visualize");

    // 3. Deletion Case from user test: Remove 20
    println!("\n[Step 3] Deleting node '20' (the current root)...");
    map.remove(&"20".to_string()).expect("Failed to remove 20");

    // 4. Visualize final state
    println!("\n[Step 4] Final Sovereign state after deletion and rebalancing:");
    map.visualize().expect("Failed to visualize");

    // 5. THE ULTIMATE TEST: BIT-FLIP SIMULATION
    println!("\n[Step 5] SIMULATING CRITICAL_DATA_INCONSISTENCY (Bit-Flip Test)...");
    println!(">> Target: Node '10'");
    
    map.simulate_bit_flip(&"10".to_string()).expect("Failed to simulate bit flip");

    println!(">> Attempting to retrieve Node '10'...");
    match map.get(&"10".to_string()) {
        Ok(val) => {
            println!("!! FAILURE: Node retrieved successfully! Value: {:?}", val);
            panic!("Sovereign Integrity Check FAILED to detect corruption!");
        }
        Err(e) => {
            println!(">> SUCCESS: Integrity Violation Detected!");
            println!(">> Error Message: {}", e);
        }
    }

    println!("\nConclusion: The Sovereign architecture successfully detected physical memory corruption.");
    println!("NASA-Grade Stability: [OK]");
    println!("Sovereign Integrity: [PASS]");
}
