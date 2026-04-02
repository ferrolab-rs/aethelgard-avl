use aethelgard_avl::SovereignMap;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_avl_property_random_mutations(ops in prop::collection::vec((0..100u32, 0..1000u32), 1..500)) {
        let map = SovereignMap::new();
        let mut shadow = std::collections::BTreeMap::new();

        for (op, val) in ops {
            let key = format!("key_{}", val);
            let value = format!("val_{}", val);

            match op % 3 {
                0 => { // Insert
                    if !shadow.contains_key(&key) {
                        map.insert(key.clone(), value.clone()).unwrap();
                        shadow.insert(key, value);
                    }
                },
                1 => { // Get & Verify
                    let res = map.get(&key).unwrap();
                    assert_eq!(res, shadow.get(&key).cloned());
                },
                2 => { // Remove
                    if shadow.contains_key(&key) {
                        map.remove(&key).unwrap();
                        shadow.remove(&key);
                    }
                },
                _ => unreachable!()
            }
            
            // Extreme Validation: Ensure every single mutation maintains all invariants
            map.validate_invariants().expect("Invariants violated during random mutation!");
        }

        // Final full audit
        map.full_audit().expect("Final integrity audit failed!");
    }
}

#[test]
fn test_complex_balancing_scenarios() {
    let map = SovereignMap::new();
    // Specific sequence that triggers multiple rotations and rebalancing
    let keys = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45, 55, 65, 75, 85];
    for k in keys {
        map.insert(k.to_string(), k.to_string()).unwrap();
    }
    
    map.full_audit().unwrap();
    
    // Balanced removal of intermediate nodes
    map.remove(&"30".to_string()).unwrap();
    map.remove(&"70".to_string()).unwrap();
    map.remove(&"50".to_string()).unwrap();
    
    map.full_audit().expect("Integrity violation after complex deletions");
}
