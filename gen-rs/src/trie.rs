use std::ops::Index;
use std::collections::HashMap;
use crate::{StrRec,Addr,SplitAddr};
use SplitAddr::{Prefix,Term};


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Trie<V> {
    leaf_nodes: HashMap<StrRec,V>,
    internal_nodes: HashMap<StrRec,Trie<V>>
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

    pub fn get_internal_node(&self, addr: StrRec) -> Option<&Self> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.internal_nodes.get(addr)
            }
            Prefix(first, rest) => {
                self.internal_nodes[first].get_internal_node(&rest)
            }
        }
    }

    pub fn insert_internal_node(&mut self, addr: StrRec, new_node: Self) -> Option<Trie<V>> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if !new_node.is_empty() {
                    self.internal_nodes.insert(addr, new_node)
                } else {
                    None
                }
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes
                    .entry(first)
                    .or_insert(Trie::new());
                node.insert_internal_node(rest, new_node)
            }
        }
    }

    pub fn remove_internal_node(&mut self, addr: StrRec) -> bool {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.internal_nodes.remove(addr);
            }
            Prefix(first, rest) => {
                if self.internal_nodes.contains_key(first) {
                    let node = self.internal_nodes.get_mut(first).unwrap();
                    if node.remove_internal_node(rest) {
                        self.internal_nodes.remove(first);
                    }
                }
            }
        }
        self.is_empty()
    }

    pub fn has_leaf_node(&self, addr: StrRec) -> bool {
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

    pub fn get_leaf_node(&self, addr: StrRec) -> Option<&V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.get(addr)
            }
            Prefix(first, rest) => {
                self.internal_nodes[first].get_leaf_node(rest)
            }
        }
    }

    pub fn insert_leaf_node(&mut self, addr: StrRec, value: V) -> Option<V> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.insert(addr, value)
            }
            Prefix(first, rest) => {
                let node = self.internal_nodes
                    .entry(first)
                    .or_insert(Trie::new());
                node.insert_leaf_node(rest, value)
            }
        }
    }

    pub fn remove_leaf_node(&mut self, addr: StrRec) -> bool {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.leaf_nodes.remove(addr);
            }
            Prefix(first, rest) => {
                if self.internal_nodes.contains_key(first) {
                    let node = self.internal_nodes.get_mut(first).unwrap();
                    if node.remove_leaf_node(rest) {
                        self.internal_nodes.remove(first);
                    }
                }
            }
        }
        self.is_empty()
    }
}

impl<V> Index<StrRec> for Trie<V> {
    type Output = V;

    fn index(&self, index: StrRec) -> &Self::Output {
        self.get_leaf_node(index).unwrap()
    }
}

impl<V> Addr for Trie<V> {
    type V = V;

    fn empty() -> Self {
        Trie::new()
    }

    fn get_submap(&self, addr: StrRec) -> Option<&Self> {
        self.get_internal_node(addr)
    }

    fn insert_submap(&mut self, addr: StrRec, submap: Self) -> Option<Self> {
        self.insert_internal_node(addr, submap)
    }

    fn get_value(&self, addr: StrRec) -> Option<&Self::V> {
        self.get_leaf_node(addr)
    }

    fn insert_value(&mut self, addr: StrRec, value: Self::V) -> Option<Self::V> {
        self.insert_leaf_node(addr, value)
    }
}