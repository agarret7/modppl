use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, hash_map};


/// Enum representing possible parse variants for an address that contain some number of `/` separators.
#[derive(Debug,PartialEq,Eq,Hash)]
pub enum SplitAddr<'a> {
    /// Resultant type from a parse of `(addr)`.
    Term(&'a str),

    /// Resultant type from a parse of `first / (addr)`.
    Prefix(&'a str, &'a str)
}
use SplitAddr::{Prefix,Term};

thread_local!(
    /// Regex spec for address parsing.
    static ADDR_RE: RefCell<Regex> = RefCell::new(Regex::new(r"^(.*?)/(.*)$").ok().unwrap())
);

impl<'a> SplitAddr<'a> {
    /// Parse a string address containing some number of `/` separators into a `SplitAddr` variant.
    pub fn from_addr(addr: &'a str) -> Self {
        match ADDR_RE.with(|re| re.borrow().captures(&addr)) {
            None => {
                Term(addr.trim_start().trim_end())
            },
            Some(caps) => {
                let first: &str = caps.get(1).unwrap().into();
                let rest: &str = caps.get(2).unwrap().into();
                Prefix(first.trim_start().trim_end(), rest)
            }
        }
    }
}

/// Normalize whitespace between `/` separators in an `addr` to contain one space to the left and right of each separator.
pub fn normalize_addr<'a>(addr: &'a str) -> String {
    match SplitAddr::from_addr(addr) {
        Term(s) => {
            s.to_string()
        }
        Prefix(first, rest) => {
            format!("{} / {}", first, normalize_addr(rest))
        }
    }
}


/// A map of strings representing a mask.
#[derive(Debug, Clone, PartialEq)]
pub struct AddrMap(HashMap<String,AddrMap>);

impl AddrMap {
    /// Construct an empty `AddrMap`.
    pub fn new() -> Self {
        AddrMap(HashMap::new())
    }

    /// Return `true` if `self` has no descendants, otherwise `false`.
    pub fn is_leaf(&self) -> bool {
        self.0.is_empty()
    }

    /// Return some reference to a descendant at `addr` if present, otherwise none.
    pub fn search(&self, addr: &str) -> Option<&AddrMap> {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.0.get(addr)
            }
            Prefix(first, rest) => {
                match self.0.get(first) {
                    Some(submask) => {
                        submask.search(rest)
                    }
                    None => { None }
                }
            }
        }
    }

    /// Insert a descendant `sub` at `addr`. Panics if `addr` is occupied.
    pub fn insert(&mut self, addr: &str, sub: AddrMap) {
        self.0.insert(addr.to_string(), sub);
    }

    /// Return `true` if for every address in `other`,
    /// `self` visited either that address or its ancestor,
    /// otherwise `false`.
    pub fn all_visited(&self, other: &AddrMap) -> bool {
        for (addr, sub) in other.iter() {
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

    /// Add an `addr` to `self`.
    pub fn visit(&mut self, addr: &str) {
        match SplitAddr::from_addr(addr) {
            Term(addr) => {
                self.0
                    .entry(addr.to_string())
                    .or_insert(AddrMap::new());
            }
            Prefix(first, rest) => {
                let submask = self.0
                    .entry(first.to_string())
                    .or_insert(AddrMap::new());
                submask.visit(rest);
            }
        }
    }

    /// Get the complement of `mask` in `self`.
    pub fn complement(&self, mask: &Self) -> Self {
        let mut cmap = AddrMap::new();
        for (addr, sub) in self.iter() {
            match mask.search(addr) {
                None => {
                    cmap.visit(addr);
                }
                Some(submask) => {
                    if !sub.is_leaf() && !submask.is_leaf() {
                        let subcomplement = sub.complement(submask);
                        if !subcomplement.is_leaf() {
                            cmap.insert(addr, subcomplement);
                        }
                    }
                }
            }
        }
        cmap
    }

    /// Iterate through the _direct_ descendants of `self`.
    pub fn iter(&self) -> hash_map::Iter<'_, String, AddrMap> {
        self.0.iter()
    }
}

#[test]
fn test_split_addr() {
    let key = SplitAddr::from_addr("test");
    assert_eq!(key, Term("test"));

    let key = SplitAddr::from_addr("(tuple, test)");
    assert_eq!(key, Term("(tuple, test)"));

    let key = SplitAddr::from_addr("1/2");
    assert_eq!(key, Prefix("1", "2"));

    let hard_addr = " 1/ 21f23/432 / 132  /   (  y?A1 , grexxy )   ";
    let mut key = SplitAddr::from_addr(hard_addr);
    assert_eq!(key, Prefix("1", " 21f23/432 / 132  /   (  y?A1 , grexxy )   "));

    while key != Term("(  y?A1 , grexxy )") {
        match key {
            Prefix(_, b) => { key = SplitAddr::from_addr(b); },
            t => { panic!("expected term = Term(\"(  y?A1 , grexxy )\"), got {:?}", t) }
        }
    }

    let equiv_addr = "1/   21f23  / 432/132 / (  y?A1 , grexxy ) ";
    let normalized_addr = "1 / 21f23 / 432 / 132 / (  y?A1 , grexxy )";
    assert_eq!(normalize_addr(hard_addr), normalized_addr);
    assert_eq!(normalize_addr(equiv_addr), normalized_addr);
}