use std::collections::{HashMap, hash_map};
use crate::{SplitAddr::{self,Prefix,Term}, AddrMap};


#[derive(Clone)]
#[derive(Debug,PartialEq)]
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

    pub fn len(&self) -> usize {
        self.mapping.len()
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

    pub fn w_observe(&mut self, addr: &str, value: V, weight: f64) { 
        self.weight += weight;
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if self.mapping.contains_key(addr) {
                    panic!("w_observe: attempted to put into occupied address \"{addr}\"");
                } else {
                    self.mapping.insert(addr.to_string(), Trie::leaf(value, weight));
                }
            }
            Prefix(first, rest) => {
                let submap = self.mapping
                    .entry(first.to_string())
                    .or_insert(Trie::new());
                submap.w_observe(rest, value, weight)
            }
        }
    }

    pub fn insert(&mut self, addr: &str, sub: Trie<V>) {
        self.weight += sub.weight;
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                if self.mapping.contains_key(addr) {
                    panic!("insert: attempted to put into occupied address \"{addr}\"");
                } else {
                    self.mapping.insert(addr.to_string(), sub);
                }
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
                self.w_observe(&addr, othersub.value.unwrap(), othersub.weight);
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

    pub fn schema(&self) -> AddrMap {
        let mut amap = AddrMap::new();
        for (addr, subtrie) in self.iter() {
            if subtrie.is_leaf() {
                amap.visit(addr);
            } else {
                amap.insert(addr, subtrie.schema());
            }
        }
        amap
    }

    pub fn collect(
        mut self: Self,
        mask: &AddrMap
    ) -> (Self,Self,f64) {
        let mut collected = Trie::new();
        if &self.schema() == mask {
            let weight = self.weight();
            return (collected, self, weight);
        } else if !mask.is_leaf() {
            for (addr, submask) in mask.iter() {
                let Some(sub) = self.remove(addr) else { unreachable!() };
                if submask.is_leaf() {
                    collected.insert(addr, sub);
                } else {
                    let (sub, subcollected, _) = sub.collect(submask);
                    if !sub.is_empty() {
                        self.insert(addr, sub);
                    }
                    if !subcollected.is_empty() {
                        collected.insert(addr, subcollected);
                    }
                }
            }
        }
        let weight = collected.weight();
        (self, collected, weight)
    }

}