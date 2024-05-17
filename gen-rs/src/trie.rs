use std::collections::{HashMap, hash_map};
use crate::SplitAddr::{self,Prefix,Term};


#[derive(Clone)]
#[derive(Debug,PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Trie<V> {
    mapping: HashMap<String,Trie<V>>,
    value: Option<V>,
    weight: f64
}


impl<V> Trie<V> {

    pub fn new() -> Self {
        Trie {
            mapping: HashMap::new(),
            value: None,
            weight: 0.
        }
    }

    pub fn leaf(value: V, weight: f64) -> Self {
        Trie {
            mapping: HashMap::new(),
            value: Some(value),
            weight: weight
        }
    }

    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty() && self.value.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.mapping.is_empty() && self.value.is_some()
    }

    pub fn ref_inner(&self) -> Option<&V> {
        self.value.as_ref()
    }

    pub fn take_inner(&mut self) -> Option<V> {
        self.value.take()
    }

    pub fn replace_inner(&mut self, value: V) -> Option<V> {
        self.value.replace(value)
    }

    pub fn expect_inner(self, msg: &str) -> V {
        self.value.expect(msg)
    }

    pub fn iter(&self) -> hash_map::Iter<'_, String, Trie<V>> {
        self.mapping.iter()
    }

    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, Trie<V>> {
        self.mapping.iter_mut()
    }

    pub fn into_iter(self) -> hash_map::IntoIter<String, Trie<V>> {
        self.mapping.into_iter()
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn search(&self, addr: &str) -> Option<&Trie<V>> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.mapping.get(addr)
            }
            Prefix(first, rest) => {
                self.mapping[first].search(rest)
            }
        }
    }

    pub fn observe(&mut self, addr: &str, value: V) {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if self.mapping.contains_key(addr) {
                    panic!("observe: attempted to put into occupied address \"{addr}\"");
                } else {
                    self.mapping.insert(addr.to_string(), Trie::leaf(value, 0.0));
                }
            }
            Prefix(first, rest) => {
                let submap = self.mapping
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                submap.observe(rest, value)
            }
        }
    }

    pub fn witness(&mut self, addr: &str, value: V, weight: f64) { 
        self.weight += weight;
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if self.mapping.contains_key(addr) {
                    panic!("witness: attempted to put into occupied address \"{addr}\"");
                } else {
                    self.mapping.insert(addr.to_string(), Trie::leaf(value, weight));
                }
            }
            Prefix(first, rest) => {
                let submap = self.mapping
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                submap.witness(rest, value, weight)
            }
        }
    }

    pub fn insert(&mut self, addr: &str, sub: Trie<V>) -> Option<Trie<V>> {
        self.weight += sub.weight;
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.mapping.insert(addr.to_string(), sub)
            }
            Prefix(first, rest) => {
                let submap = self.mapping
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                submap.insert(rest, sub)
            }
        }
    }

    pub fn remove(&mut self, addr: &str) -> Option<Trie<V>> {
        if let Some(sub) = match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.mapping.remove(addr)
            }
            Prefix(first, rest) => {
                match self.mapping.get_mut(first) {
                    Some(node) => {
                        let leaf = node.remove(rest);
                        if node.is_empty() {
                            self.remove(first);
                        }
                        leaf
                    }
                    None => { None }
                }
            }
        } {
            self.weight -= sub.weight;
            Some(sub)
        } else {
            None
        }
    }

    pub fn merge(&mut self, other: Self) {
        for (addr, othersub) in other.into_iter() {
            if othersub.is_leaf() {
                self.witness(&addr, othersub.value.unwrap(), othersub.weight);
            } else {
                match self.mapping.get_mut(&addr) {
                    Some(sub) => {
                        sub.merge(othersub);
                    }
                    None => {
                        self.insert(&addr, othersub);
                    }
                }
            }
        }
    }

}


/// Trie for hierarchical address schemas (with empty values).
pub type AddrTrie = Trie<()>;

impl AddrTrie {

    /// Return the unique `AddrTrie` that contains an `addr` if and only if `data` contains that `addr`.
    pub fn schema<V>(data: &Trie<V>) -> Self {
        let mut visitor = Trie::new();
        for (addr, sub) in data.iter() {
            visitor.insert(addr, Self::schema(sub));
        }
        visitor
    }

    /// Add an address to the `AddrTrie`.
    pub fn visit(&mut self, addr: &str) {
        self.observe(addr, ());
    }

    /// Return `true` if every `addr` in `data` is also present in `self`,
    /// and every leaf of `self` is also a leaf of `data`.
    pub fn all_visited<V>(&self, data: &Trie<V>) -> bool {
        for (addr, sub) in data.iter() {
            if let Some(subvisitor) = self.search(&addr) {
                if !subvisitor.is_leaf() && !subvisitor.all_visited(sub) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    }

    /// Return the `AddrTrie` that contains all addresses present in `data`, but not present in `self`,
    /// where each address that is a leaf of `data`
    pub fn get_unvisited<V>(&self, data: &Trie<V>) -> Self {
        let mut unvisited = Trie::new();
        for (addr, sub) in data.iter() {
            match self.search(addr) {
                None => {
                    unvisited.visit(addr);
                }
                Some(subvisitor) => {
                    if !sub.is_leaf() && !subvisitor.is_leaf() {
                        unvisited.insert(addr, subvisitor.get_unvisited(sub));
                    }
                }
            }
        }
        unvisited
    }

}