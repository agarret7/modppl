use std::any::Any;
use std::{hash::Hash, fmt::Display, ops::{Index,IndexMut}, rc::Rc};
use crate::gfi::{Addr, ChoiceVal, ChoiceBuffer};
use std::collections::HashMap;


pub struct ChoiceHashMap<V: ChoiceVal> {
    hmap: HashMap<Addr,Rc<V>>
}

impl<V: ChoiceVal> ChoiceHashMap<V> {
    pub fn new() -> Self {
        ChoiceHashMap { hmap: HashMap::new() }
    }
}

impl<V: ChoiceVal> Index<Addr> for ChoiceHashMap<V> {
    type Output = Rc<V>;

    fn index(&self, k: Addr) -> &Self::Output {
        (self.get_value(k) as &dyn Any).downcast_ref::<Rc<V>>().unwrap()
    }
}

impl<V: ChoiceVal> Clone for ChoiceHashMap<V> {
    fn clone(&self) -> Self {
        let mut choices = ChoiceHashMap::new();
        for k in self.hmap.keys().into_iter() {
            choices.set_value(k, &Rc::clone(&self.hmap[k]));
        }
        choices
    }
}

impl<V: ChoiceVal> ChoiceBuffer for ChoiceHashMap<V> {
    fn has_value(&self, k: Addr) -> bool {
        self.hmap.contains_key(&k)
    }

    fn get_value(&self, k: Addr) -> &Rc<impl ChoiceVal> {
        &self.hmap[&k]
    }

    fn set_value(&mut self, k: Addr, v: &Rc<impl ChoiceVal>) {
        self.hmap.insert(k, Rc::clone((v as &dyn Any).downcast_ref::<Rc<V>>().unwrap()));
    }
}