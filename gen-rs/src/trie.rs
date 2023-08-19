use std::ops::Index;
use std::collections::{HashMap, hash_map};
use crate::SplitAddr::{self,Prefix,Term};


/// Hierarchical prefix tree
#[derive(Debug,Clone,PartialEq)]
pub struct Trie<V> {
    leaf_nodes: HashMap<String,V>,
    internal_nodes: HashMap<String,Trie<V>>
}

impl<V> Trie<V> {
    /// Construct an empty Trie.
    pub fn new() -> Self {
        Trie {
            leaf_nodes: HashMap::new(),
            internal_nodes: HashMap::new()
        }
    }

    /// Return `true` if a Trie is empty (has no leaf or internal nodes), otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.leaf_nodes.is_empty() && self.internal_nodes.is_empty()
    }

    /// Return `true` if a Trie has a leaf node at `addr`, otherwise `false`.
    pub fn has_leaf_node(&self, addr: &str) -> bool {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.contains_key(addr)
            }
            Prefix(first, rest) => {
                if self.internal_nodes.contains_key(first) {
                    self.internal_nodes[first].has_leaf_node(rest)
                } else {
                    false
                }
            }
        }
    }

    /// Return `Some(&value)` if `self` contains a `value` located at `addr`, otherwise `None`.
    pub fn get_leaf_node(&self, addr: &str) -> Option<&V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.get(addr)
            }
            Prefix(first, rest) => {
                self.internal_nodes[first].get_leaf_node(rest)
            }
        }
    }

    /// Insert `value` as a leaf node located at `addr`.
    /// 
    /// If there was a value `prev` located at `addr`, return `Some(prev)`, otherwise `None`.
    pub fn insert_leaf_node(&mut self, addr: &str, value: V) -> Option<V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.insert(addr.to_string(), value)
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                node.insert_leaf_node(rest, value)
            }
        }
    }

    /// Return `Some(value)` if `self` contains a `value` located at `addr` and remove `value` from the leaf nodes, otherwise return `None`.
    pub fn remove_leaf_node(&mut self, addr: &str) -> Option<V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.remove(addr)
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes.get_mut(first).unwrap();
                let leaf = node.remove_leaf_node(rest);
                if node.is_empty() {
                    self.remove_internal_node(first);
                }
                leaf
            }
        }
    }

    /// Return an iterator over a Trie's leaf nodes.
    pub fn leaf_iter(&self) -> hash_map::Iter<'_, String, V> {
        self.leaf_nodes.iter()
    }

    /// Return `true` if a Trie has an internal node at `addr`, otherwise `false`.
    pub fn has_internal_node(&self, addr: &str) -> bool {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.internal_nodes.contains_key(addr)
            }
            Prefix(first, rest) => {
                if self.internal_nodes.contains_key(first) {
                    self.internal_nodes[first].has_internal_node(rest)
                } else {
                    false
                }
            }
        }
    }

    /// Return `Some(&subtrie)` if `self` contains a `subtrie` located at `addr`, otherwise `None`.
    pub fn get_internal_node(&self, addr: &str) -> Option<&Self> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.internal_nodes.get(addr)
            }
            Prefix(first, rest) => {
                self.internal_nodes[first].get_internal_node(rest)
            }
        }
    }

    /// Insert `subtrie` as an internal node located at `addr`.
    /// 
    /// If there was a value `prev_subtrie` located at `addr`, return `Some(prev_subtrie)`, otherwise `None`.
    /// Panics if `subtrie.is_empty()`.
    pub fn insert_internal_node(&mut self, addr: &str, new_node: Self) -> Option<Trie<V>> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if !new_node.is_empty() {
                    self.internal_nodes.insert(addr.to_string(), new_node)
                } else {
                    panic!("attempted to insert empty inode")
                }
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                node.insert_internal_node(rest, new_node)
            }
        }
    }

    /// Return `Some(subtrie)` if `self` contains a `subtrie` located at `addr` and remove `subtrie` from the internal nodes, otherwise return `None`.
    pub fn remove_internal_node(&mut self, addr: &str) -> Option<Trie<V>> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.internal_nodes.remove(addr)
            }
            Prefix(first, _) => {
                self.internal_nodes.remove(first)
            }
        }
    }

    /// Return an iterator over a Trie's internal nodes.
    pub fn internal_iter(&self) -> hash_map::Iter<'_, String, Trie<V>> {
        self.internal_nodes.iter()
    }

    /// Merge `other` into `self`, freeing previous values and subtries at each `addr` in `self` if `other` also has an entry at `addr`.
    /// 
    /// Returns the mutated `self`.
    pub fn merge(mut self, other: Self) -> Self {
        for (addr, value) in other.leaf_nodes.into_iter() {
            self.insert_leaf_node(&addr, value);
        }
        for (addr, subtrie) in other.internal_nodes.into_iter() {
            self.insert_internal_node(&addr, subtrie);
        }
        self
    }
}


// specializations

impl<V> Trie<(V,f64)> {
    /// Return the sum of all the weights of the leaf nodes and the recursive sum of all internal nodes.
    pub fn sum(&self) -> f64 {
        self.internal_nodes.values().fold(0., |acc, t| acc + t.sum()) +
        self.leaf_nodes.values().fold(0., |acc, v| acc + v.1)
    }

    /// Convert a weighted `Trie` into the equivalent unweighted version by discarding all the weights.
    pub fn into_unweighted(self) -> Trie<V> {
        Trie {
            internal_nodes: self.internal_nodes.into_iter().map(|(addr, t)| (addr, t.into_unweighted())).collect::<_>(),
            leaf_nodes: self.leaf_nodes.into_iter().map(|(addr, v)| (addr, v.0)).collect::<_>()
        }
    }

    /// Convert an unweighted `Trie` into the equivalent weighted version by adding a weight of `0.` to all leaf nodes.
    pub fn from_unweighted(trie: Trie<V>) -> Self {
        Trie {
            internal_nodes: trie.internal_nodes.into_iter().map(|(addr, t)| (addr, Self::from_unweighted(t))).collect::<_>(),
            leaf_nodes: trie.leaf_nodes.into_iter().map(|(addr, v)| (addr, (v, 0.))).collect::<_>()
        }
    }
}

use std::{rc::Rc,any::Any};

impl Trie<Rc<dyn Any>> {
    /// Optimistically casts the reference-counted `dyn Any` at `addr` into type `V`, and returns a cloned value.
    pub fn read<V: 'static + Clone>(&self, addr: &str) -> V {
        self.get_leaf_node(addr)
            .unwrap()
            .clone()
            .downcast::<V>()
            .ok()
            .unwrap()
            .as_ref()
            .clone()
    }
}

impl Trie<(Rc<dyn Any>,f64)> {
    /// Optimistically casts the reference-counted `dyn Any` at `addr` into type `V`, and returns a cloned value.
    pub fn read<V: 'static + Clone>(&self, addr: &str) -> V {
        self.get_leaf_node(addr)
            .unwrap().0
            .clone()
            .downcast::<V>()
            .ok()
            .unwrap()
            .as_ref()
            .clone()
    }
}

impl<V> Index<&str> for Trie<V> {
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        self.get_leaf_node(index).unwrap()
    }
}