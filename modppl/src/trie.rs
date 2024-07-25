use std::collections::{HashMap, hash_map};
use crate::{SplitAddr::{self,Prefix,Term}, AddrMap};


/// Weighted Digital Trie
#[derive(Debug,Clone,PartialEq)]
pub struct Trie<V> {
    mapping: HashMap<String,Trie<V>>,
    value: Option<V>,
    weight: f64
}


impl<V> Trie<V> {

    /// Initialize an empty Trie.
    pub fn new() -> Self {
        Trie {
            mapping: HashMap::new(),
            value: None,
            weight: 0.
        }
    }

    /// Initialize a Trie with an inner value and weight.
    pub fn leaf(value: V, weight: f64) -> Self {
        Trie {
            mapping: HashMap::new(),
            value: Some(value),
            weight: weight
        }
    }

    /// Return `true` if `self` is empty (has no inner value nor descendants), otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty() && self.value.is_none()
    }

    /// Return `true` if `self` is a leaf (has an inner value but no descendants), otherwise `false`.
    pub fn is_leaf(&self) -> bool {
        self.mapping.is_empty() && self.value.is_some()
    }

    /// Return the number of _direct_ descendants of the `Trie`.
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Return some reference to the inner value if there is one, otherwise none.
    pub fn ref_inner(&self) -> Option<&V> {
        self.value.as_ref()
    }

    /// Return some inner value (setting the inner value to none), otherwise just return none.
    pub fn take_inner(&mut self) -> Option<V> {
        self.value.take()
    }

    /// Return some inner value (setting the inner value to `value`), otherwise just return none.
    pub fn replace_inner(&mut self, value: V) -> Option<V> {
        self.value.replace(value)
    }

    /// Return some inner value if there is one, otherwise panic with `msg`.
    pub fn expect_inner(self, msg: &str) -> V {
        self.value.expect(msg)
    }

    /// Iterate through the _direct_ descendants of `self`.
    pub fn iter(&self) -> hash_map::Iter<'_, String, Trie<V>> {
        self.mapping.iter()
    }

    /// Iterate mutably through the _direct_ descendants of `self`.
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, String, Trie<V>> {
        self.mapping.iter_mut()
    }

    /// Move `self` into an iterator over the _direct_ descendants of `self`.
    pub fn into_iter(self) -> hash_map::IntoIter<String, Trie<V>> {
        self.mapping.into_iter()
    }

    /// Return the sum of the weight of all descendants.
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Return some reference to a descendant at `addr` if present, otherwise none.
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

    /// Observe an unweighted `value` at `addr`. Panic if `addr` is occupied.
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

    /// Observe a weighted `value` at `addr`, summing the weight by `weight`. Panic if `addr` is occupied.
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

    /// Insert a descendant `sub` at `addr`. Panic if `addr` is occupied.
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

    /// Return a descendant at `addr` if present (removing it), otherwise just return none.
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

    /// Merge an `other` Trie into `self`, preferentially using the values of `other` at overlapping addresses.
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

    /// Return an `AddrMap` representing the address schema of `self`.
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

    /// Collect the set of values identified by `mask` into a new `Trie`,
    /// leaving values in `self` that are in the complement of `mask`.
    /// 
    /// Return the new `self`, the collected value trie, and the weight of the collected value trie.
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