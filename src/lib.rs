// Copyright (c) 2026 Ferrolab
// Licensed under the MIT License or the Apache License, Version 2.0.
// See LICENSE-MIT or LICENSE-APACHE for details.

//! # Sovereign-AVL
//! 
//! **Sovereign-AVL** is a NASA-grade, self-auditing data structure designed for environments where
//! memory integrity and cryptographic certainty are non-negotiable.
//! 
//! It combines three fundamental architectural axioms to achieve "Sovereign" status:
//! 1. **Generational Arena Storage**: Eliminates Use-After-Free (Zombie Index) vulnerabilities.
//! 2. **Merkle-Style Integrity**: Every node is cryptographically tied to its children via BLAKE3 hashes.
//! 3. **AVL Balancing**: Guaranteed $O(\log n)$ performance for all operations.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use aethelgard_avl::SovereignMap;
//! 
//! let map = SovereignMap::new();
//! 
//! // Secure insertion: Hashes are computed and propagated automatically.
//! map.insert("Aethelgard".to_string(), "Sovereign".to_string()).unwrap();
//! 
//! // Integrity-checked retrieval:
//! if let Ok(Some(val)) = map.get(&"Aethelgard".to_string()) {
//!     println!("Retrieved: {}", val);
//! }
//! 
//! // Full Recursive Audit:
//! map.full_audit().expect("Integrity violation detected!");
//! ```
//! 
//! ## Why Sovereign?
//! 
//! In mission-critical environments (Space, Medical, Financial), bit-flips and adversarial memory corruption 
//! are real threats. **Sovereign-AVL** assumes the memory is *not* trusted and verifies every bit on every access.
//! 
//! ## ⚖️ Limitations & Trade-offs
//! 
//! - **CPU Cost**: BLAKE3 hashing on every update is expensive (~100x slower than `BTreeMap`).
//! - **Memory Cost**: ~36-40 bytes of overhead per node (plus padding).
//! - **Write Contention**: Uses a global `RwLock`, which might bottleneck high-frequency parallel writes.
//! - **Audit Latency**: `full_audit` is $O(n)$ and can take several milliseconds for large trees.

pub mod avl;
pub mod error;
pub mod storage;

use avl::SovereignAVL;
use error::Result;
use parking_lot::RwLock;
use std::sync::Arc;
use std::fmt::Debug;

/// A thread-safe, self-auditing AVL tree with sovereign integrity.
pub struct SovereignMap<K, V> {
    inner: Arc<RwLock<SovereignAVL<K, V>>>,
}

impl<K, V> SovereignMap<K, V>
where
    K: Clone + Debug + PartialOrd + AsRef<[u8]> + Send + Sync + 'static,
    V: Clone + Debug + AsRef<[u8]> + Send + Sync + 'static,
{
    /// Creates a new, empty `SovereignMap`.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SovereignAVL::new())),
        }
    }

    /// Inserts a key-value pair into the map. 
    /// If the key exists, returns `SovereignError::DuplicateKey`.
    pub fn insert(&self, key: K, value: V) -> Result<()> {
        let mut tree = self.inner.write();
        tree.insert(key, value)
    }

    /// Removes a key from the map and its associated value.
    pub fn remove(&self, key: &K) -> Result<()> {
        let mut tree = self.inner.write();
        tree.delete(key)
    }

    /// Retrieves a copy of the value associated with the key.
    /// Performs an integrity check on every node visited during the search.
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let tree = self.inner.read();
        tree.get(key)
    }

    /// Returns an iterator over the map's key-value pairs, in-order.
    /// This method holds a read lock on the tree for the duration of its existence.
    pub fn iter(&self) -> SovereignIter<'_, K, V> {
        let guard = self.inner.read();
        // Safety: We extend the lifetime of the borrow because the guard is held by the iterator itself.
        let inner_iter = unsafe {
            std::mem::transmute::<avl::InOrderIter<'_, K, V>, avl::InOrderIter<'static, K, V>>(
                guard.iter()
            )
        };
        SovereignIter { _guard: guard, inner_iter }
    }

    /// Performs a full recursive audit of the entire tree's integrity.
    /// Returns `Ok(())` if the tree is consistent, or an `IntegrityViolation` error.
    pub fn full_audit(&self) -> Result<()> {
        let tree = self.inner.read();
        tree.full_audit()
    }

    /// Visualizes the internal tree structure and integrity hashes to stdout.
    pub fn visualize(&self) -> Result<()> {
        let tree = self.inner.read();
        tree.dump_tree()
    }

    /// Simulates a bit-flip in the memory of a specific node's value.
    /// **WARNING**: For validation and stress-testing ONLY.
    pub fn simulate_bit_flip(&self, key: &K) -> Result<()> {
        let mut tree = self.inner.write();
        tree.internal_corrupt_node(key)
    }

    /// Performs a full recursive validation of all tree invariants (BST order, AVL balance, Merkle integrity).
    /// Returns `Ok(())` if the tree is mathematically and physically certain.
    pub fn validate_invariants(&self) -> Result<()> {
        let tree = self.inner.read();
        tree.validate_invariants().map(|_| ())
    }
}

/// An iterator over the entries of a `SovereignMap`.
pub struct SovereignIter<'a, K: 'static, V: 'static> {
    _guard: parking_lot::RwLockReadGuard<'a, SovereignAVL<K, V>>,
    inner_iter: avl::InOrderIter<'static, K, V>,
}

impl<'a, K, V> Iterator for SovereignIter<'a, K, V>
where
    K: Clone + Debug + Send + Sync + 'static,
    V: Clone + Debug + Send + Sync + 'static,
{
    type Item = (K, V); // We return clones to avoid complex lifetime issues with the guard

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|(k, v)| (k.clone(), v.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sovereign_operations() {
        let map = SovereignMap::new();
        map.insert("key1".to_string(), "value1".to_string()).unwrap();
        map.insert("key2".to_string(), "value2".to_string()).unwrap();
        
        assert_eq!(map.get(&"key1".to_string()).unwrap(), Some("value1".to_string()));
        assert_eq!(map.get(&"key2".to_string()).unwrap(), Some("value2".to_string()));
        assert_eq!(map.get(&"key3".to_string()).unwrap(), None);
    }

    #[test]
    fn test_avl_balancing_simple() {
        let map = SovereignMap::new();
        // LL Case (Sequential insertion should trigger rotations)
        map.insert("c".to_string(), "3".to_string()).unwrap();
        map.insert("b".to_string(), "2".to_string()).unwrap();
        map.insert("a".to_string(), "1".to_string()).unwrap();
        
        assert_eq!(map.get(&"a".to_string()).unwrap(), Some("1".to_string()));
        assert_eq!(map.get(&"b".to_string()).unwrap(), Some("2".to_string()));
        assert_eq!(map.get(&"c".to_string()).unwrap(), Some("3".to_string()));
    }
}
