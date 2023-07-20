use crate::{Trie,Trace};

// why store a Rc<V> instead of V?
// - Traces can share memory across particles, requiring dramatically less space
// - Indirection in choice values 

enum TraceRecord<V> {
    Choice { retval: Rc<V>, score: f64 },
    Call { subtrace: Rc<V>, score: f64, noise: f64 }
}

struct DynamicDSLTrace<X,T,V: Any> {
    trie: Trie<TraceRecord<V>>,
    is_empty: bool,
    score: f64,
    noise: f64,
    args: X,
    retval: Option<T>
}

impl<X,T,V> DynamicDSLTrace<X,T,V> {
    pub fn new(args: X) -> Self {
        let trie = Trie::<TraceRecord<V>>::new();
        DynamicDSLTrace {
            trie,
            is_empty: true,
            score: 0.,
            noise: 0.,
            args,
            retval: None
        }
    }

    pub fn set_retval(&mut self, retval: T) {
        self.retval = Some(retval);
    }
}

struct DynamicChoiceTrie<V> {
    trie: Trie<TraceRecord<V>>
}

impl<X,T,V> Trace for DynamicDSLTrace<X,T,V> {
    type X = X;
    type T = T;

    fn get_args(&self) -> &Self::X { &self.args }
    fn get_retval(&self) -> &Self::T { &self.retval.unwrap() }

    fn get_score(&self) -> f64 { self.score }

    fn set_score(&mut self, new_score: f64) { self.score = new_score; }

    fn get_choices(&self) -> DynamicDSLTrace {
        
    }
}