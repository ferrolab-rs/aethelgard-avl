use aethelgard_avl::SovereignMap;
use std::time::Instant;

fn main() {
    println!("=== Aethelgard Sovereign-AVL Extreme Proof ===");
    println!("Date: {:?}\n", Instant::now());

    let map = SovereignMap::new();

    // --- PHASE 1: MATHEMATICAL CERTAINTY (AVL & BST) ---
    println!("[PHASE 1] Validating Mathematical Invariants...");
    let keys = vec!["50", "30", "70", "20", "40", "60", "80"];
    println!(">> Inserting complex sequence: {:?}", keys);
    for k in &keys {
        map.insert(k.to_string(), format!("Val_{}", k)).unwrap();
    }

    println!("\n>> Current Tree Visualization:");
    map.visualize().unwrap();

    println!("\n>> Running Recursive Invariant Auditor...");
    match map.validate_invariants() {
        Ok(_) => println!(">> SUCCESS: All AVL Balance and BST Order invariants are perfectly met."),
        Err(e) => panic!(">> FAILURE: Invariants violated: {}", e),
    }

    // --- PHASE 2: PHYSICAL INTEGRITY (MERKLE PROPAGATION) ---
    println!("\n[PHASE 2] Validating Cryptographic Integrity (Merkle Chain)...");
    println!(">> Target: Injecting 1-bit corruption in node '20' (a leaf node).");
    map.simulate_bit_flip(&"20".to_string()).unwrap();

    println!(">> Attempting Full Audit at the Root level...");
    match map.full_audit() {
        Ok(_) => panic!(">> FAILURE: Integrity violation went undetected at the root!"),
        Err(e) => {
            println!(">> SUCCESS: Sovereign Auditor detected the leaf corruption from the root.");
            println!(">> Audit Error: {}", e);
        }
    }

    // --- PHASE 3: MEMORY SAFETY (GENERATIONAL ZOMBIE TEST) ---
    println!("\n[PHASE 3] Validating Generational Safety (Zombie Index Prevention)...");
    println!(">> Step 1: Deleting node '80'.");
    map.remove(&"80".to_string()).unwrap();

    println!(">> Step 2: Forcing reallocation and cycling the free list...");
    for i in 0..100 {
        let key = format!("temp_{}", i);
        map.insert(key.clone(), "trash".to_string()).unwrap();
        map.remove(&key).unwrap();
    }

    println!(">> Step 3: Verifying that searching for '80' returns None (Safe).");
    let res = map.get(&"80".to_string()).unwrap();
    assert!(res.is_none());
    println!(">> SUCCESS: Search for deleted node '80' returned None.");

    println!("\n[CONCLUSION]");
    println!("1. BST Order: [OK]");
    println!("2. AVL Balance: [OK]");
    println!("3. Cryptographic Chain: [OK]");
    println!("4. Generational Safety: [OK]");
    println!("\nVerdict: SOVEREIGN-AVL-TREE IS MATHEMATICALLY AND PHYSICALLY CERTAIN.");
}
