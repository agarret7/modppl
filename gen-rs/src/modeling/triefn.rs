use std::{collections::HashSet, rc::Rc};
use rand::rngs::ThreadRng;

// use super::{Rc<dyn Any>};
use std::any::Any;
use crate::modeling::dists::Distribution;
use crate::{Trie, GenFn, GfDiff, Trace, StrRec};

pub enum TrieFnState<'a,A,T> {
    Simulate { trace: Trace<A,Trie<Rc<dyn Any>>,T>, weight_trie: Trie<f64> },
    Generate { trace: Trace<A,Trie<Rc<dyn Any>>,T>, weight_trie: Trie<f64>, weight: f64, constraints: Trie<Rc<dyn Any>> },
    Update {
        trace: &'a mut Trace<A,Trie<Rc<dyn Any>>,T>,
        constraints: Trie<Rc<dyn Any>>,
        weight_trie: Trie<f64>,
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

impl<'a,A: 'static,T: 'static> TrieFnState<'a,A,T> {
    pub fn sample_at<
        V: Clone + 'static,
        W: Clone + 'static
    >(&mut self, dist: &mut impl Distribution<V,W>, args: W, addr: StrRec) -> V {
        match self {
            TrieFnState::Simulate {
                trace,
                weight_trie
            } => {
                let x = dist.random(&mut dist.rng(), args.clone());
                let logp = dist.logpdf(&x, args);
                trace.logp += logp;
                weight_trie.insert_leaf_node(addr, logp);
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, Rc::new(x.clone()));
                x
            }
            TrieFnState::Generate {
                trace,
                weight_trie,
                weight,
                constraints
            } => {
                let (x, dlogp) = match constraints.remove_leaf_node(addr) {
                    None => {
                        let x = dist.random(&mut dist.rng(), args.clone());
                        let dlogp = dist.logpdf(&x, args);
                        (Rc::new(x), dlogp)
                    }
                    Some(call) => {
                        let x = call.downcast::<V>().ok().unwrap();
                        let dlogp = dist.logpdf(x.as_ref(), args);
                        *weight += dlogp;
                        (x, dlogp)
                    }
                };
                trace.logp += dlogp;
                weight_trie.insert_leaf_node(addr, dlogp);
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, x.clone());
                x.as_ref().clone()
            }
            TrieFnState::Update {
                trace,
                constraints,
                weight_trie,
                weight,
                discard
            } => {
                let data = trace.get_data_mut();
                let prev_retv: Rc<V>;
                let retv: Rc<V>;

                let has_previous = data.has_leaf_node(addr);
                let constrained = constraints.has_leaf_node(addr);
                let mut prev_logp = 0.;
                if has_previous {
                    prev_retv = data.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                    prev_logp = 0.;  // todo: replace logp from previous choice
                    if constrained {
                        discard.insert_leaf_node(addr, prev_retv);
                        retv = constraints.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                    } else {
                        retv = prev_retv;
                    }
                } else {
                    if constrained {
                        retv = constraints.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                    } else {
                        retv = Rc::new(dist.random(&mut dist.rng(), args.clone()));
                    }
                }

                let logp = dist.logpdf(retv.as_ref(), args);
                if has_previous {
                    *weight += logp - prev_logp;
                } else {
                    *weight += logp;
                }

                data.insert_leaf_node(addr, retv.clone());
                weight_trie.insert_leaf_node(addr, logp);

                retv.as_ref().clone()
            }
        }
    }

    // trace a random call, storing the subtrace as a black-box heap-allocated leaf node
    pub fn trace_at<
        X: 'static,
        Y: Clone + 'static,
    >(&mut self, mut gen_fn: impl GenFn<X,Trie<Rc<dyn Any>>,Y>, args: X, addr: StrRec) -> Y {
        match self {
            TrieFnState::Simulate {
                trace,
                weight_trie
            } => {
                let subtrace = gen_fn.simulate(args);
                trace.logp += subtrace.logp;
                let retv = subtrace.get_retv().unwrap().clone();
                let data = trace.get_data_mut();
                data.insert_internal_node(addr, subtrace.data);
                retv
            }
            TrieFnState::Generate {
                trace,
                weight_trie,
                weight,
                constraints
            } => {
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
                weight_trie,
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


impl<Args: Clone + 'static,Ret: 'static> GenFn<Args,Trie<Rc<dyn Any>>,Ret> for TrieFn<Args,Ret> {
    fn rng(&self) -> ThreadRng { self.rng.clone() }

    fn simulate(&mut self, args: Args) -> Trace<Args,Trie<Rc<dyn Any>>,Ret> {
        let mut state = TrieFnState::Simulate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
            weight_trie: Trie::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Simulate {mut trace, weight_trie} = state else { unreachable!() };
        trace.set_retv(retv);
        trace
    }

    fn generate(&mut self, args: Args, constraints: Trie<Rc<dyn Any>>) -> (Trace<Args,Trie<Rc<dyn Any>>,Ret>, f64) {
        let mut state = TrieFnState::Generate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
            weight_trie: Trie::new(),
            weight: 0.,
            constraints
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Generate {mut trace, weight_trie, weight, constraints} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&mut self,
        trace: &mut Trace<Args,Trie<Rc<dyn Any>>,Ret>,
        args: Args,
        _: GfDiff,
        constraints: Trie<Rc<dyn Any>>
    ) -> (Trie<Rc<dyn Any>>, f64) {
        let mut state = TrieFnState::Update {
            trace: trace,
            weight_trie: Trie::new(),
            weight: 0.,
            constraints,
            discard: Trie::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Update {mut trace, weight_trie, weight, constraints, discard} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (discard, weight)
    }
}