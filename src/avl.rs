use crate::error::{Result, SovereignError};
use crate::storage::{NodeContainer, NULL_INDEX, RawIndex};
use blake3::Hasher;
use std::fmt::Debug;

pub struct SovereignAVL<K, V> {
    storage: NodeContainer<K, V>,
    root: RawIndex,
}

impl<K, V> SovereignAVL<K, V>
where
    K: Clone + Debug + PartialOrd + AsRef<[u8]>,
    V: Clone + Debug + AsRef<[u8]>,
{
    pub fn new() -> Self {
        Self {
            storage: NodeContainer::new(),
            root: NULL_INDEX,
        }
    }

    // --- Core Algorithmic Logic (SET_D) ---

    fn get_height(&self, id: RawIndex) -> i32 {
        if id == NULL_INDEX {
            0
        } else {
            self.storage.get(id).map(|n| n.height).unwrap_or(0)
        }
    }

    fn update_height(&mut self, id: RawIndex) -> Result<()> {
        let left_h = self.get_height(self.storage.get(id)?.left);
        let right_h = self.get_height(self.storage.get(id)?.right);
        self.storage.get_mut(id)?.height = 1 + std::cmp::max(left_h, right_h);
        
        // Sovereign Mandate: Update Hash on Height Update
        self.update_hash(id)?;
        Ok(())
    }

    fn get_balance(&self, id: RawIndex) -> Result<i32> {
        if id == NULL_INDEX {
            Ok(0)
        } else {
            let node = self.storage.get(id)?;
            Ok(self.get_height(node.left) - self.get_height(node.right))
        }
    }

    // --- Rotations (SET_D: Balanced Search Tree) ---

    fn rotate_right(&mut self, y: RawIndex) -> Result<RawIndex> {
        let x = self.storage.get(y)?.left;
        let t2 = self.storage.get(x)?.right;

        // Perform rotation
        self.storage.get_mut(x)?.right = y;
        self.storage.get_mut(y)?.left = t2;

        // Update heights
        self.update_height(y)?;
        self.update_height(x)?;

        Ok(x)
    }

    fn rotate_left(&mut self, x: RawIndex) -> Result<RawIndex> {
        let y = self.storage.get(x)?.right;
        let t2 = self.storage.get(y)?.left;

        // Perform rotation
        self.storage.get_mut(y)?.left = x;
        self.storage.get_mut(x)?.right = t2;

        // Update heights
        self.update_height(x)?;
        self.update_height(y)?;

        Ok(y)
    }

    // --- Sovereign Hashing Layer ---

    fn update_hash(&mut self, id: RawIndex) -> Result<()> {
        let mut hasher = Hasher::new();
        let node = self.storage.get(id)?;

        // Payload hashing
        hasher.update(node.key.as_ref());
        hasher.update(node.value.as_ref());
        
        // Child hashing (Integrity chain)
        if node.left != NULL_INDEX {
            hasher.update(&self.storage.get(node.left)?.hash.to_le_bytes());
        }
        if node.right != NULL_INDEX {
            hasher.update(&self.storage.get(node.right)?.hash.to_le_bytes());
        }

        let hash_bytes = hasher.finalize();
        let mut hash_val = 0u64;
        for i in 0..8 {
            hash_val |= (hash_bytes.as_bytes()[i] as u64) << (i * 8);
        }
        
        self.storage.get_mut(id)?.hash = hash_val;
        Ok(())
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<()> {
        self.root = self.recursive_insert(self.root, key, value)?;
        Ok(())
    }

    fn recursive_insert(&mut self, id: RawIndex, key: K, value: V) -> Result<RawIndex> {
        if id == NULL_INDEX {
            let nid = self.storage.allocate(key, value)?;
            self.update_hash(nid)?;
            return Ok(nid);
        }

        let current_key = self.storage.get(id)?.key.clone();
        if key < current_key {
            let left = self.storage.get(id)?.left;
            let new_left = self.recursive_insert(left, key.clone(), value.clone())?;
            self.storage.get_mut(id)?.left = new_left;
        } else if key > current_key {
            let right = self.storage.get(id)?.right;
            let new_right = self.recursive_insert(right, key.clone(), value.clone())?;
            self.storage.get_mut(id)?.right = new_right;
        } else {
            return Err(SovereignError::DuplicateKey);
        }

        self.update_height(id)?;
        let balance = self.get_balance(id)?;

        // LL Case
        if balance > 1 && key < self.storage.get(self.storage.get(id)?.left)?.key {
            return self.rotate_right(id);
        }

        // RR Case
        if balance < -1 && key > self.storage.get(self.storage.get(id)?.right)?.key {
            return self.rotate_left(id);
        }

        // LR Case
        if balance > 1 && key > self.storage.get(self.storage.get(id)?.left)?.key {
            let left = self.storage.get(id)?.left;
            self.storage.get_mut(id)?.left = self.rotate_left(left)?;
            return self.rotate_right(id);
        }

        // RL Case
        if balance < -1 && key < self.storage.get(self.storage.get(id)?.right)?.key {
            let right = self.storage.get(id)?.right;
            self.storage.get_mut(id)?.right = self.rotate_right(right)?;
            return self.rotate_left(id);
        }

        Ok(id)
    }

    pub fn get(&self, key: &K) -> Result<Option<V>> {
        self.recursive_get(self.root, key)
    }

    fn recursive_get(&self, id: RawIndex, key: &K) -> Result<Option<V>> {
        if id == NULL_INDEX {
            return Ok(None);
        }

        // Sovereign Integrity: Probability-based verification or simple on-get
        // In this implementation, we always verify the node we are visiting.
        self.verify_integrity(id)?;

        let node = self.storage.get(id)?;
        if *key < node.key {
            self.recursive_get(node.left, key)
        } else if *key > node.key {
            self.recursive_get(node.right, key)
        } else {
            Ok(Some(node.value.clone()))
        }
    }

    pub fn delete(&mut self, key: &K) -> Result<()> {
        self.root = self.recursive_delete(self.root, key)?;
        Ok(())
    }

    fn recursive_delete(&mut self, id: RawIndex, key: &K) -> Result<RawIndex> {
        if id == NULL_INDEX {
            return Ok(NULL_INDEX);
        }

        let current_id = id;
        let node_key = self.storage.get(current_id)?.key.clone();

        if *key < node_key {
            let left = self.storage.get(current_id)?.left;
            let new_left = self.recursive_delete(left, key)?;
            self.storage.get_mut(current_id)?.left = new_left;
        } else if *key > node_key {
            let right = self.storage.get(current_id)?.right;
            let new_right = self.recursive_delete(right, key)?;
            self.storage.get_mut(current_id)?.right = new_right;
        } else {
            // Node found
            let left = self.storage.get(current_id)?.left;
            let right = self.storage.get(current_id)?.right;

            if left == NULL_INDEX || right == NULL_INDEX {
                let temp = if left != NULL_INDEX { left } else { right };

                if temp == NULL_INDEX {
                    // No child case
                    self.storage.deallocate(current_id)?;
                    return Ok(NULL_INDEX);
                } else {
                    // One child case
                    self.storage.deallocate(current_id)?;
                    return Ok(temp);
                }
            } else {
                // Two children case: Get successor
                let successor_id = self.get_min_node(right)?;
                let successor_key = self.storage.get(successor_id)?.key.clone();
                let successor_val = self.storage.get(successor_id)?.value.clone();

                // Swap data
                self.storage.get_mut(current_id)?.key = successor_key.clone();
                self.storage.get_mut(current_id)?.value = successor_val;

                // Delete successor
                let new_right = self.recursive_delete(right, &successor_key)?;
                self.storage.get_mut(current_id)?.right = new_right;
            }
        }

        self.update_height(current_id)?;
        let balance = self.get_balance(current_id)?;

        // LL Case
        if balance > 1 && self.get_balance(self.storage.get(current_id)?.left)? >= 0 {
            return self.rotate_right(current_id);
        }

        // LR Case
        if balance > 1 && self.get_balance(self.storage.get(current_id)?.left)? < 0 {
            let left = self.storage.get(current_id)?.left;
            self.storage.get_mut(current_id)?.left = self.rotate_left(left)?;
            return self.rotate_right(current_id);
        }

        // RR Case
        if balance < -1 && self.get_balance(self.storage.get(current_id)?.right)? <= 0 {
            return self.rotate_left(current_id);
        }

        // RL Case
        if balance < -1 && self.get_balance(self.storage.get(current_id)?.right)? > 0 {
            let right = self.storage.get(current_id)?.right;
            self.storage.get_mut(current_id)?.right = self.rotate_right(right)?;
            return self.rotate_left(current_id);
        }

        Ok(current_id)
    }

    fn get_min_node(&self, id: RawIndex) -> Result<RawIndex> {
        let mut curr = id;
        while curr != NULL_INDEX {
            let left = self.storage.get(curr)?.left;
            if left == NULL_INDEX {
                break;
            }
            curr = left;
        }
        Ok(curr)
    }

    pub fn dump_tree(&self) -> Result<()> {
        println!("--- Sovereign Tree State ---");
        self.recursive_dump(self.root, 0)?;
        println!("----------------------------");
        Ok(())
    }

    fn recursive_dump(&self, id: RawIndex, depth: usize) -> Result<()> {
        if id == NULL_INDEX {
            return Ok(());
        }
        let node = self.storage.get(id)?;
        println!("{:indent$}[Key: {:?}, Height: {}, Hash: {:016x}]", "", node.key, node.height, node.hash, indent=depth*2);
        self.recursive_dump(node.left, depth + 1)?;
        self.recursive_dump(node.right, depth + 1)?;
        Ok(())
    }

    /// Internal method to simulate a bit-flip in the node's memory.
    /// ONLY FOR VALIDATION.
    pub fn internal_corrupt_node(&mut self, key: &K) -> Result<()> {
        let mut curr = self.root;
        while curr != NULL_INDEX {
            let node = self.storage.get(curr)?;
            if *key < node.key {
                curr = node.left;
            } else if *key > node.key {
                curr = node.right;
            } else {
                // Node found - Perform bit flip in value using unsafe raw access
                let node_mut = self.storage.get_mut(curr)?;
                let data_ptr = node_mut.value.as_ref().as_ptr() as *mut u8;
                unsafe {
                    *data_ptr ^= 0x01; 
                }
                return Ok(());
            }
        }
        Err(SovereignError::NodeNotFound(0))
    }

    pub fn verify_integrity(&self, id: RawIndex) -> Result<()> {
        let node = self.storage.get(id)?;
        let mut hasher = Hasher::new();
        hasher.update(node.key.as_ref());
        hasher.update(node.value.as_ref());
        
        if node.left != NULL_INDEX {
            hasher.update(&self.storage.get(node.left)?.hash.to_le_bytes());
        }
        if node.right != NULL_INDEX {
            hasher.update(&self.storage.get(node.right)?.hash.to_le_bytes());
        }

        let hash_bytes = hasher.finalize();
        let mut current_hash = 0u64;
        for i in 0..8 {
            current_hash |= (hash_bytes.as_bytes()[i] as u64) << (i * 8);
        }

        if current_hash != node.hash {
            return Err(SovereignError::IntegrityViolation(id));
        }
        Ok(())
    }

    /// Returns an in-order iterator over the tree.
    pub fn iter(&self) -> InOrderIter<'_, K, V> {
        InOrderIter::new(self.root, &self.storage)
    }

    /// Performs a full recursive audit of the entire tree's integrity.
    pub fn full_audit(&self) -> Result<()> {
        self.recursive_audit(self.root)
    }

    fn recursive_audit(&self, id: RawIndex) -> Result<()> {
        if id == NULL_INDEX {
            return Ok(());
        }
        self.verify_integrity(id)?;
        let node = self.storage.get(id)?;
        self.recursive_audit(node.left)?;
        self.recursive_audit(node.right)?;
        Ok(())
    }

    /// Recursively validates all AVL and BST invariants for the entire tree.
    /// Returns `Ok(())` if all invariants (Order, Height, Balance, Hash) are perfectly met.
    pub fn validate_invariants(&self) -> Result<()> {
        self.recursive_validate_invariants(self.root, None, None).map(|_| ())
    }

    fn recursive_validate_invariants(&self, id: RawIndex, min: Option<&K>, max: Option<&K>) -> Result<i32> {
        if id == NULL_INDEX {
            return Ok(0);
        }

        let node = self.storage.get(id)?;
        
        // 1. BST Invariant: G < R < D
        if let Some(m) = min {
            if node.key <= *m { return Err(SovereignError::IntegrityViolation(id)); }
        }
        if let Some(m) = max {
            if node.key >= *m { return Err(SovereignError::IntegrityViolation(id)); }
        }

        // 2. Merkle Integrity: Immediate verification
        self.verify_integrity(id)?;

        // 3. Recursive Checks
        let left_h = self.recursive_validate_invariants(node.left, min, Some(&node.key))?;
        let right_h = self.recursive_validate_invariants(node.right, Some(&node.key), max)?;

        // 4. AVL Invariant: Balance factor
        if (left_h - right_h).abs() > 1 {
            return Err(SovereignError::IntegrityViolation(id));
        }

        // 5. Stored Height Invariant
        let actual_h = 1 + std::cmp::max(left_h, right_h);
        if node.height != actual_h {
            return Err(SovereignError::IntegrityViolation(id));
        }

        Ok(actual_h)
    }
}


pub struct InOrderIter<'a, K, V> {
    stack: Vec<RawIndex>,
    storage: &'a NodeContainer<K, V>,
}

impl<'a, K, V> InOrderIter<'a, K, V>
where
    K: Clone + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    fn new(root: RawIndex, storage: &'a NodeContainer<K, V>) -> Self {
        let mut stack = Vec::new();
        let mut curr = root;
        while curr != NULL_INDEX {
            stack.push(curr);
            curr = storage.get(curr).map(|n| n.left).unwrap_or(NULL_INDEX);
        }
        Self { stack, storage }
    }
}

impl<'a, K, V> Iterator for InOrderIter<'a, K, V>
where
    K: Clone + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node_id = self.stack.pop()?;
        let node = self.storage.get(node_id).ok()?;
        
        let mut next_node = node.right;
        while next_node != NULL_INDEX {
            self.stack.push(next_node);
            next_node = self.storage.get(next_node).map(|n| n.left).unwrap_or(NULL_INDEX);
        }
        
        Some((&node.key, &node.value))
    }
}

