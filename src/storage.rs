use crate::error::{Result, SovereignError};
use std::fmt::Debug;

pub type RawIndex = u32;
pub const NULL_INDEX: RawIndex = u32::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIndex {
    pub id: RawIndex,
    pub generation: u32,
}

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: RawIndex,
    pub right: RawIndex,
    pub parent: RawIndex,
    pub height: i32,
    pub hash: u64,
    pub generation: u32,
    pub is_active: bool,
}

pub struct NodeContainer<K, V> {
    nodes: Vec<Node<K, V>>,
    free_list: Vec<RawIndex>,
}

impl<K, V> NodeContainer<K, V>
where
    K: Clone + Debug,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: Vec::with_capacity(64),
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self, key: K, value: V) -> Result<RawIndex> {
        if let Some(id) = self.free_list.pop() {
            let node = &mut self.nodes[id as usize];
            
            // NASA-Grade: Overflow protection for generation
            if node.generation == u32::MAX {
                return Err(SovereignError::CapacityExceeded);
            }
            
            node.generation += 1;
            node.key = key;
            node.value = value;
            node.left = NULL_INDEX;
            node.right = NULL_INDEX;
            node.parent = NULL_INDEX;
            node.height = 1;
            node.hash = 0;
            node.is_active = true;
            Ok(id)
        } else {
            let id = self.nodes.len() as RawIndex;
            self.nodes.push(Node {
                key,
                value,
                left: NULL_INDEX,
                right: NULL_INDEX,
                parent: NULL_INDEX,
                height: 1,
                hash: 0,
                generation: 1,
                is_active: true,
            });
            Ok(id)
        }
    }

    pub fn deallocate(&mut self, id: RawIndex) -> Result<()> {
        let node = self.get_mut(id)?;
        node.is_active = false;
        self.free_list.push(id);
        Ok(())
    }

    pub fn get(&self, id: RawIndex) -> Result<&Node<K, V>> {
        if id == NULL_INDEX || id as usize >= self.nodes.len() {
            return Err(SovereignError::NodeNotFound(id));
        }
        let node = &self.nodes[id as usize];
        if !node.is_active {
            return Err(SovereignError::StaleIndex);
        }
        Ok(node)
    }

    pub fn get_mut(&mut self, id: RawIndex) -> Result<&mut Node<K, V>> {
        if id == NULL_INDEX || id as usize >= self.nodes.len() {
            return Err(SovereignError::NodeNotFound(id));
        }
        let node = &mut self.nodes[id as usize];
        if !node.is_active {
            return Err(SovereignError::StaleIndex);
        }
        Ok(node)
    }
}
