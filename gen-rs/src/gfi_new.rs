// The core insight is that optimizing a probabilistic program is really about optimizing
// searchable _memory structure_ given deterministic constraints on particular (known) data queries.

// Thus, constraints on generative function DSLs express constraints on the structure type
// used to represent call and choice values on subtraces.

// In Gen.jl:
// - A StaticDSL program infers optimized codegen for structs and constructors, given flat randomness.
// - A DynamicDSL program infers optimized trie constructors, given recursive hierarchical randomness.
// - The Unfold combinator infers optimized vector constructors, given sequential randomness.

// Reflecting this pattern, gen-rs forgoes indirection via Choice and Trace interfaces,
// directly exposing data type as parameterizing the domain specific language.

// This expands the {Static | Dynamic} DSLs into the {struct | HashMap, Trie*, MerkleTree & Vec**} DSLs.
// *Trie is the default when annotations are omitted.
// **Vec replaces the Unfold combinator from Gen.jl.

// GenMerkle demos
// Verify two large traces contain the same data

// Since each of these structures can implement the Index trait,
// choices are represented as implementations of the Index trait (`impl Index`).

// A user can compose languages to express an eg. "array-of-tries" pattern or
// "trie-of-structs"

// gen!(Trie fn trie_model(x: f64, y: f64) -> bool {
//     z ~ bernoulli(-(y - x).abs().exp());
// });

// gen!(Vec fn model(T: int) -> Vec<bool> {
//     let a ~ beta(2., 5.);
//     let bs = vec![]
//     {i} = (1..T).into_iter() {
//         let t = i;
//         dbg!(t);
//         let x = {x} ~ normal(0, a);
//         let y = {y} ~ normal(a, a^2);
//         let b = {b} ~ trie_model(x, y);
//         bs.push(b);
//     }
//     bs
// });


use rand::rngs::ThreadRng;
use std::ops::Index;
use std::any::Any;


pub type StrRec = &'static str;

pub trait Addr: Index<StrRec> + Sized {
    type V;
    fn empty() -> Self;
    fn get_submap(&self, addr: StrRec) -> Option<&Self>;
    fn insert_submap(&mut self, addr: StrRec, submap: Self) -> Option<Self>;
    fn get_value(&self, addr: StrRec) -> Option<&Self::V>;
    fn insert_value(&mut self, addr: StrRec, value: Self::V) -> Option<Self::V>;
}

pub struct Sample<T>(pub T);

impl<T> Index<StrRec> for Sample<T> {
    type Output = T;

    fn index(&self, _: StrRec) -> &Self::Output {
        &self.0
    }
}

impl<T> Addr for Sample<T> {
    type V = T;
    fn empty() -> Self { panic!("samples can't be empty") }
    fn get_submap(&self, _: StrRec) -> Option<&Self> { panic!("samples don't have submaps") }
    fn insert_submap(&mut self, _: StrRec, _: Self) -> Option<Self> { panic!("samples don't have submaps") }
    fn get_value(&self, _: StrRec) -> Option<&Self::V> { Some(&self.0) }
    fn insert_value(&mut self, _: StrRec, value: Self::V) -> Option<Self::V> {
        Some(std::mem::replace(&mut self.0, value))
    }
}

pub struct Trace<A,D: Addr,T> {
    args: A,
    pub data: D,
    retv: Option<T>,
    pub logp: f64
}

impl<A: 'static,D: Addr + 'static,T: 'static> Trace<A,D,T> {
    pub fn new(args: A, data: D, retv: T, logp: f64) -> Self {
        Trace { args, data, retv: Some(retv), logp }
    }

    pub fn empty(args: A) -> Self {
        Trace {
            args,
            data: D::empty(),
            retv: None,
            logp: 0.
        }
    }

    pub fn get_args(&self) -> &A { &self.args }
    pub fn get_data(&self) -> &D { &self.data }
    pub fn get_data_mut(&mut self) -> &mut D { &mut self.data }
    pub fn get_retv(&self) -> Option<&T> { self.retv.as_ref() }
    pub fn set_retv(&mut self, v: T) { self.retv = Some(v); }
    pub fn logpdf(&self) -> f64 { self.logp }
}

// ~ TraceBox ~
//   "What's inside the box?"

//   Dynamic type erasure gives flexibility over the types of samples,
//   and a unified language interface for sampling from other generative
//   functions, while guaranting strong protection against memory leaks.
//   This _requires_ all choice values live on the heap and are dynamically typed, however.
pub struct TraceBox {
    args: Box<dyn Any>,
    data: Box<dyn Any>,
    retv: Option<Box<dyn Any>>,
    pub logp: f64
}

impl TraceBox {
    pub fn from_trace<A: 'static,D: Addr + 'static,T: 'static>(trace: Trace<A,D,T>) -> Self {
        TraceBox {
            args: Box::new(trace.args),
            data: Box::new(trace.data),
            retv: match trace.retv {
                None => None,
                Some(v) => Some(Box::new(v))
            },
            logp: trace.logp
        }
    }

    pub fn into_inner<A: 'static,D: Addr + 'static,T: 'static>(self) -> Trace<A,D,T> {
        Trace {
            args: *self.args.downcast::<A>().ok().unwrap(),
            data: *self.data.downcast::<D>().ok().unwrap(),
            retv: match self.retv {
                None => None,
                Some(v)=> Some(*v.downcast::<T>().ok().unwrap())
            },
            logp: self.logp
        }
    }
}

pub trait GenFn<A,D: Addr,T> {

    fn rng(&self) -> ThreadRng;
    fn simulate(&mut self, args: A) -> Trace<A,D,T>;
    fn generate(&mut self, args: A, constraints: impl Addr) -> (Trace<A,D,T>, f64);

    fn update(&mut self,
        trace: &mut Trace<A,D,T>,
        args: A,
        diff: crate::GfDiff,
        constraints: impl Addr  // forward choices
    ) -> (D, f64);      // backward choices

    // fn call(&mut self, args: Self::A) -> Self::T;
    // fn propose(&mut self, args: Self::A) -> (impl Addr, f64);
    // fn assess(&mut self, args: Self::A, constraints: impl Addr) -> f64;

}


// TODO: extend the semantics to support variable-length input and per-argument diffs.
#[derive(Debug,Clone)]
pub enum GfDiff {
    NoChange,
    Unknown,
    Extend
}