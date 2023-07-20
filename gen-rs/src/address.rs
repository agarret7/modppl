use regex::Regex;
use crate::StrRec;


#[derive(Debug,PartialEq,Eq,Hash)]
pub enum SplitAddr {
    Term(StrRec),
    Prefix(StrRec, StrRec)
}
use SplitAddr::{Prefix,Term};


use std::time::Instant;

impl SplitAddr {
    pub fn from_addr(addr: StrRec) -> Self {
        let re: Regex = Regex::new(r"^(.*?)=>(.*)$").ok().unwrap();
        match re.captures(&addr) {
            None => {
                Term(addr.trim_start().trim_end())
            },
            Some(caps) => {
                let first: StrRec = caps.get(1).unwrap().into();
                let rest: StrRec = caps.get(2).unwrap().into();
                Prefix(first.trim_start().trim_end(), rest)
            }
        }
    }
}


#[test]
fn test_trie_key() {
    let key = SplitAddr::from_addr("test");
    assert_eq!(key, Term("test"));

    let key = SplitAddr::from_addr("(tuple, test)");
    assert_eq!(key, Term("(tuple, test)"));

    let key = SplitAddr::from_addr("1=>2");
    assert_eq!(key, Prefix("1", "2"));

    let hard_addr = " 1=> 21f23=>432 => 132  =>   (  y?A1 , grexxy )   ";
    let mut key = SplitAddr::from_addr(hard_addr);
    assert_eq!(key, Prefix("1", " 21f23=>432 => 132  =>   (  y?A1 , grexxy )   "));

    while key != Term("(  y?A1 , grexxy )") {
        match key {
            Prefix(a, b) => { key = SplitAddr::from_addr(b); },
            t => { panic!("expected term = Term(\"(  y?A1 , grexxy )\"), got {:?}", t) }
        }
    }
}