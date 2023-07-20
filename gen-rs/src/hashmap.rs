use std::collections::HashMap;
use crate::{StrRec,Addr,SplitAddr};


impl<V: Clone> Addr for HashMap<StrRec,V> {
    type V = V;

    fn empty() -> Self {
        Self::new()
    }

    fn get_submap(&self, _: StrRec) -> Option<&Self> {
        Some(self)
    }

    fn insert_submap(&mut self, _: StrRec, submap: Self) -> Option<Self> {
        let mut discard = Self::new();
        for (k, v) in submap.into_iter() {
            match self.insert(k, v) {
                None => { },
                Some(v) => { discard.insert(k, v); }
            }
        }
        if !discard.is_empty() { Some(discard) } else { None }
    }

    fn get_value(&self, addr: StrRec) -> Option<&Self::V> {
        self.get(addr)
    }

    fn insert_value(&mut self, addr: StrRec, value: Self::V) -> Option<Self::V> {
        self.insert(addr, value)
    }
}