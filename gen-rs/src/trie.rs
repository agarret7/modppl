use std::ops::Index;
use std::collections::{HashMap, hash_map};
use crate::{SplitAddr};
use SplitAddr::{Prefix,Term};


#[derive(Debug,Clone,PartialEq)]
pub struct Trie<V> {
    leaf_nodes: HashMap<String,V>,
    internal_nodes: HashMap<String,Trie<V>>
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Trie {
            leaf_nodes: HashMap::new(),
            internal_nodes: HashMap::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.leaf_nodes.is_empty() && self.internal_nodes.is_empty()
    }

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

    pub fn remove_leaf_node(&mut self, addr: &str) -> Option<V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.remove(addr)
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes.get_mut(first).unwrap();
                node.remove_leaf_node(rest)
            }
        }
    }

    pub fn leaf_iter(&self) -> hash_map::Iter<'_, String, V> {
        self.leaf_nodes.iter()
    }

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

    pub fn insert_internal_node(&mut self, addr: &str, new_node: Self) -> Option<Trie<V>> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if !new_node.is_empty() {
                    self.internal_nodes.insert(addr.to_string(), new_node)
                } else {
                    None
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

    pub fn internal_iter(&self) -> hash_map::Iter<'_, String, Trie<V>> {
        self.internal_nodes.iter()
    }

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

impl<V> Trie<(V,f64)> {
    pub fn sum(&self) -> f64 {
        self.internal_nodes.values().fold(0., |acc, t| acc + t.sum()) +
        self.leaf_nodes.values().fold(0., |acc, v| acc + v.1)
    }

    pub fn into_unweighted(self) -> Trie<V> {
        Trie {
            internal_nodes: self.internal_nodes.into_iter().map(|(addr, t)| (addr, t.into_unweighted())).collect::<_>(),
            leaf_nodes: self.leaf_nodes.into_iter().map(|(addr, v)| (addr, v.0)).collect::<_>()
        }
    }

    pub fn from_unweighted(trie: Trie<V>) -> Self {
        Trie {
            internal_nodes: trie.internal_nodes.into_iter().map(|(addr, t)| (addr, Self::from_unweighted(t))).collect::<_>(),
            leaf_nodes: trie.leaf_nodes.into_iter().map(|(addr, v)| (addr, (v, 0.))).collect::<_>()
        }
    }
}

impl<V> Index<&str> for Trie<V> {
    type Output = V;

    fn index(&self, index: &str) -> &Self::Output {
        self.get_leaf_node(index).unwrap()
    }
}