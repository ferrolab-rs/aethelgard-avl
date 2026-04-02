use aethelgard_avl::SovereignMap;
use aethelgard_avl::error::SovereignError;

#[test]
fn test_generational_zombie_prevention() {
    let map = SovereignMap::new();
    
    // 1. Insert and get key
    map.insert("A".to_string(), "ValueA".to_string()).unwrap();
    assert!(map.get(&"A".to_string()).is_ok());

    // 2. Delete key
    map.remove(&"A".to_string()).unwrap();
    
    // 3. Verify it's gone
    assert_eq!(map.get(&"A".to_string()).unwrap(), None);

    // 4. Stress the allocator to reuse the same RAW index but with different generation
    // We insert and delete many keys to cycle through the free list
    for i in 0..100 {
        let key = format!("temp_{}", i);
        map.insert(key.clone(), "temp".to_string()).unwrap();
        map.remove(&key).unwrap();
    }

    // 5. Final check: The system should be stable and invariants should hold
    map.validate_invariants().expect("Invariants violated after allocation stress");
}

#[test]
fn test_duplicate_key_prevention() {
    let map = SovereignMap::new();
    map.insert("dup".to_string(), "v1".to_string()).unwrap();
    
    let res = map.insert("dup".to_string(), "v2".to_string());
    assert!(matches!(res, Err(SovereignError::DuplicateKey)));
}
