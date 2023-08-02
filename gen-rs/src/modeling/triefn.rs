// type TrieTrace<A,T> = StackTrace<A,Addr<,T>;
// type VecTrace<A,D: Addr<str>,T> = StackTrace<A,Vec<D>,Vec<T>>;

// struct TrieFn<A,T>();

// struct TrieBuilder {
// }

use std::collections::HashSet;
use std::rc::Rc;

use rand::rngs::ThreadRng;

// use super::{Rc<dyn Any>};
use std::any::Any;
use crate::modeling::dists::Distribution;
use crate::{Trie, GenFn, GfDiff, Trace, StrRec, Sample};

pub enum TrieFnState<A,T> {
    Simulate { trace: Trace<A,Trie<Rc<dyn Any>>,T> },
    Generate { trace: Trace<A,Trie<Rc<dyn Any>>,T>, weight: f64, constraints: Trie<Rc<dyn Any>> },
    Update {
        trace: Trace<A,Trie<Rc<dyn Any>>,T>,
        constraints: Trie<Rc<dyn Any>>,
        weight: f64,
        discard: Trie<Rc<dyn Any>>
    }
}

struct AddressVisitor {
    visited: HashSet<StrRec>
}

impl AddressVisitor {
    fn visit(&mut self, addr: StrRec) {
    }

    fn all_visited<T>(&self, data: Trie<T>) -> bool {
        false
    }

    fn get_unvisited<T>(&self, data: Trie<T>) -> HashSet<T> {
        panic!("unimplemented")
    }
}

impl<A: 'static,T: 'static> TrieFnState<A,T> {
    pub fn sample_at<
        V: Clone + 'static,
        W: Clone + 'static
    >(&mut self, dist: &mut impl Distribution<V,W>, args: W, addr: StrRec) -> V {
        match self {
            TrieFnState::Simulate { ref mut trace } => {
                let x = dist.random(&mut dist.rng(), args.clone());
                trace.logp += dist.logpdf(&x, args);
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, Rc::new(x.clone()));
                x
            }
            TrieFnState::Generate { ref mut trace, ref mut weight, constraints } => {
                let (x, dp) = match constraints.remove_leaf_node(addr) {
                    None => {
                        let x = dist.random(&mut dist.rng(), args.clone());
                        let dp = dist.logpdf(&x, args);
                        (Rc::new(x), dp)
                    }
                    Some(call) => {
                        let x = call.downcast::<V>().ok().unwrap();
                        let dp = dist.logpdf(x.as_ref(), args);
                        *weight += dp;
                        (x, dp)
                    }
                };
                trace.logp += dp;
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, x.clone());
                x.as_ref().clone()
            }
            TrieFnState::Update {
                trace,
                constraints,
                weight,
                discard
            } => {
                panic!()
                // let data = trace.get_data_mut();
                // let mut prev_score = 0.;
                // let prev_choice: Rc<V>;
                // let prev_retv: V;

                // let has_previous = data.has_leaf_node(addr);
                // if has_previous {
                //     prev_choice = data.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                //     prev_score = 0.;
                // }
            }
        }
    }

    // trace a random call, storing the subtrace as a black-box heap-allocated leaf node
    pub fn trace_at<
        X: 'static,
        Y: Clone + 'static,
    >(&mut self, mut gen_fn: impl GenFn<X,Trie<Rc<dyn Any>>,Y>, args: X, addr: StrRec) -> Y {
        match self {
            TrieFnState::Simulate { ref mut trace } => {
                let subtrace = gen_fn.simulate(args);
                trace.logp += subtrace.logp;
                let retv = subtrace.get_retv().unwrap().clone();
                let data = trace.get_data_mut();
                data.insert_internal_node(addr, subtrace.data);
                retv
            }
            TrieFnState::Generate { ref mut trace, weight, constraints } => {
                let subtrace = match constraints.remove_internal_node(addr) {
                    None => {
                        gen_fn.simulate(args)
                    }
                    Some(subconstraints) => {
                        let (subtrace, new_weight) = gen_fn.generate(args, subconstraints);
                        *weight += new_weight;
                        subtrace
                    }
                };
                trace.logp += subtrace.logp;
                let retv = subtrace.get_retv().unwrap().clone();
                let data = trace.get_data_mut();
                data.insert_internal_node(addr, subtrace.data);
                retv
            },
            TrieFnState::Update {
                trace,
                constraints,
                weight,
                discard
            } => {
                panic!()
            }
        }
    }
}

pub struct TrieFn<A,T> {
    rng: ThreadRng,
    func: fn(&mut TrieFnState<A,T>, A) -> T,
}

impl<Args,Ret> TrieFn<Args,Ret>{
    pub fn new(func: fn(&mut TrieFnState<Args,Ret>, Args) -> Ret) -> Self {
        TrieFn {
            rng: ThreadRng::default(),
            func
        }
    }
}

// A DynGenFn is a generative function that constructs its stack-trace during
// runtime (allocating all internal memory dynamically on the heap).
impl<Args: Clone + 'static,Ret: 'static> GenFn<Args,Trie<Rc<dyn Any>>,Ret> for TrieFn<Args,Ret> {
    fn rng(&self) -> ThreadRng { self.rng.clone() }

    fn simulate(&mut self, args: Args) -> Trace<Args,Trie<Rc<dyn Any>>,Ret> {
        let mut state = TrieFnState::Simulate { trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. } };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Simulate {mut trace} = state else { unreachable!() };
        trace.set_retv(retv);
        trace
    }

    fn generate(&mut self, args: Args, constraints: Trie<Rc<dyn Any>>) -> (Trace<Args,Trie<Rc<dyn Any>>,Ret>, f64) {
        // dbg!(constraints.is_empty());
        // dbg!(&constraints);
        let mut state = TrieFnState::Generate { trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. }, weight: 0., constraints };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Generate {mut trace, weight, constraints} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        // dbg!(constraints.is_empty());
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&mut self,
        mut trace: Trace<Args,Trie<Rc<dyn Any>>,Ret>,
        args: Args,
        diff: GfDiff,
        constraints: Trie<Rc<dyn Any>>
    ) -> (Trie<Rc<dyn Any>>, f64) {
        let mut state = TrieFnState::Update {
            trace: trace,
            weight: 0.,
            constraints,
            discard: Trie::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Update {mut trace, weight, constraints, discard} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (discard, weight)
    }

    // fn call(&mut self, args: Self::A) -> Self::T;
    // fn propose(&mut self, args: Self::A) -> (impl Addr<str>, f64);
    // fn assess(&mut self, args: Self::A, constraints: impl Addr<str>) -> f64;
}