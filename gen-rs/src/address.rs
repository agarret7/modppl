use regex::Regex;
use std::cell::RefCell;


#[derive(Debug,PartialEq,Eq,Hash)]
pub enum SplitAddr<'a> {
    Term(&'a str),
    Prefix(&'a str, &'a str)
}
use SplitAddr::{Prefix,Term};

thread_local!(static RE: RefCell<Regex> = RefCell::new(Regex::new(r"^(.*?)=>(.*)$").ok().unwrap()));

impl<'a> SplitAddr<'a> {
    pub fn from_addr(addr: &'a str) -> Self {
        match RE.with(|re| re.borrow().captures(&addr)) {
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

    pub fn normalize(addr: &'a str) -> String {
        match Self::from_addr(addr) {
            Term(s) => {
                s.to_string()
            }
            Prefix(first, rest) => {
                let mut first = first.to_owned();
                first.push_str(&Self::normalize(rest));
                first
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
            Prefix(_, b) => { key = SplitAddr::from_addr(b); },
            t => { panic!("expected term = Term(\"(  y?A1 , grexxy )\"), got {:?}", t) }
        }
    }
}