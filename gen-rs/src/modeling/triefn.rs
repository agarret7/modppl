use std::{collections::HashSet, rc::Rc};
use rand::rngs::ThreadRng;

// use super::{Rc<dyn Any>};
use std::any::Any;
use crate::modeling::dists::Distribution;
use crate::{Trie, GenFn, GfDiff, Trace, SplitAddr, normalize_addr};

pub enum TrieFnState<'a,A,T> {
    Simulate {
        trace: Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
        visitor: AddressVisitor
    },
    Generate {
        trace: Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
        weight: f64,
        constraints: Trie<Rc<dyn Any>>,
        visitor: AddressVisitor
    },
    Update {
        trace: &'a mut Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
        constraints: Trie<Rc<dyn Any>>,
        weight: f64,
        discard: Trie<Rc<dyn Any>>,
        visitor: AddressVisitor
    }
}

pub struct AddressVisitor {
    visited: HashSet<String>
}

impl AddressVisitor {
    pub fn new() -> Self {
        AddressVisitor { visited: HashSet::new() }
    }

    pub fn visit(&mut self, addr: &str) {
        self.visited.insert(normalize_addr(addr));
    }

    pub fn all_visited<T>(&self, data: Trie<T>) -> bool {
        false
        // for addr in data.leaf_addrs() {
        //     if !self.visited.contains(addr) {
        //         false
        //     }
        // }
        // for addr in data.internal_addrs() {
        //     self.all_visited(data.get_internal_node(addr))
        // }
    }

    pub fn get_unvisited<T>(&self, data: Trie<T>) -> HashSet<T> {
        panic!("unimplemented")
    }
}

impl<'a,A: 'static,T: 'static> TrieFnState<'a,A,T> {
    pub fn sample_at<
        V: Clone + 'static,
        W: Clone + 'static
    >(&mut self, dist: &mut impl Distribution<V,W>, args: W, addr: &str) -> V {
        match self {
            TrieFnState::Simulate {
                trace,
                visitor
            } => {
                visitor.visit(addr);

                let x = dist.random(&mut dist.rng(), args.clone());
                let logp = dist.logpdf(&x, args);
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, (Rc::new(x.clone()), logp));
                trace.logp += logp;
                x
            }

            TrieFnState::Generate {
                trace,
                weight,
                constraints,
                visitor
            } => {
                visitor.visit(addr);

                // check if there are constraints
                let (x, logp) = match constraints.remove_leaf_node(addr) {
                    // if None, sample a value and calculate change to trace.logp
                    None => {
                        let x = dist.random(&mut dist.rng(), args.clone());
                        let logp = dist.logpdf(&x, args);
                        (Rc::new(x), logp)
                    }
                    // if Some, cast to type V, calculate change to trace.logp (and add to weight)
                    Some(call) => {
                        let x = call.downcast::<V>().ok().unwrap();
                        let logp = dist.logpdf(x.as_ref(), args);
                        *weight += logp;
                        (x, logp)
                    }
                };
                
                // mutate trace with sampled leaf, increment total trace.logp, and insert in logp_trie.
                let data = trace.get_data_mut();
                data.insert_leaf_node(addr, (x.clone(), logp));
                trace.logp += logp;

                x.as_ref().clone()
            }

            TrieFnState::Update {
                trace,
                constraints,
                weight,
                discard,
                visitor
            } => {
                visitor.visit(addr);

                let data = trace.get_data_mut();
                let prev_x: Rc<V>;
                let x: Rc<V>;

                let has_previous = data.has_leaf_node(addr);
                let constrained = constraints.has_leaf_node(addr);
                let mut prev_logp = 0.;
                if has_previous {
                    let val = data.remove_leaf_node(addr).unwrap();
                    prev_x = val.0.downcast::<V>().ok().unwrap();
                    prev_logp = val.1;
                    if constrained {
                        discard.insert_leaf_node(addr, prev_x);
                        x = constraints.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                    } else {
                        x = prev_x;
                    }
                } else {
                    if constrained {
                        x = constraints.remove_leaf_node(addr).unwrap().downcast::<V>().ok().unwrap();
                    } else {
                        x = Rc::new(dist.random(&mut dist.rng(), args.clone()));
                    }
                }

                let logp = dist.logpdf(x.as_ref(), args);
                if has_previous {
                    *weight += logp - prev_logp;
                } else {
                    *weight += logp;
                }

                data.insert_leaf_node(addr, (x.clone(), logp));

                x.as_ref().clone()
            }
        }
    }

    pub fn trace_at<
        X: Clone + 'static,
        Y: Clone + 'static,
    >(&mut self, mut gen_fn: impl GenFn<X,Trie<(Rc<dyn Any>,f64)>,Y>, args: X, addr: &str) -> Y {
        match self {
            TrieFnState::Simulate {
                trace,
                visitor
            } => {
                visitor.visit(addr);

                let subtrace = gen_fn.simulate(args);

                let data = trace.get_data_mut();
                data.insert_internal_node(addr, subtrace.data);

                let retv = subtrace.retv.unwrap();
                data.insert_leaf_node(addr, (Rc::new(retv.clone()), subtrace.logp));
                trace.logp += subtrace.logp;

                retv
            }

            TrieFnState::Generate {
                trace,
                weight,
                constraints,
                visitor
            } => {
                visitor.visit(addr);

                let subtrace = match constraints.remove_internal_node(addr) {
                    None => {
                        gen_fn.simulate(args)
                    }
                    Some(subconstraints) => {
                        let (subtrace, new_weight) = gen_fn.generate(args, Trie::from_unweighted(subconstraints));
                        *weight += new_weight;
                        subtrace
                    }
                };

                let data = trace.get_data_mut();
                data.insert_internal_node(addr, subtrace.data);

                let retv = subtrace.retv.unwrap().clone();
                data.insert_leaf_node(addr, (Rc::new(retv.clone()), subtrace.logp));
                trace.logp += subtrace.logp;

                retv
            },

            TrieFnState::Update {
                trace,
                constraints,
                weight,
                discard,
                visitor
            } => {
                visitor.visit(addr);

                let data = trace.get_data_mut();
                let prev_subtrie: Trie<(Rc<dyn Any>,f64)>;
                let subtrie: Trie<(Rc<dyn Any>,f64)>;
                let retv: Rc<Y>;

                let has_previous = data.has_internal_node(addr);
                let constrained = constraints.has_internal_node(addr);
                let mut logp = 0.;
                if has_previous {
                    prev_subtrie = data.remove_internal_node(addr).unwrap();
                    if constrained {
                        let subconstraints = Trie::from_unweighted(constraints.remove_internal_node(addr).unwrap());
                        let prev_logp = prev_subtrie.sum();
                        let mut trace = Trace { args: args.clone(), data: prev_subtrie, retv: None, logp: prev_logp };
                        let out = gen_fn.update(&mut trace, args, GfDiff::Unknown, subconstraints);
                        discard.insert_internal_node(addr, out.0.unweighted());
                        subtrie = trace.data;
                        retv = Rc::new(trace.retv.unwrap());
                        logp = out.1;
                    } else {
                        subtrie = prev_subtrie;
                        retv = data.remove_leaf_node(addr).unwrap().0.downcast::<Y>().ok().unwrap();
                    }
                } else {
                    if constrained {
                        let subconstraints = Trie::from_unweighted(constraints.remove_internal_node(addr).unwrap());
                        let out = gen_fn.generate(args, subconstraints);
                        let subtrace = out.0;
                        subtrie = subtrace.data;
                        retv = Rc::new(subtrace.retv.unwrap());
                        logp = out.1;
                    } else {
                        let subtrace = gen_fn.simulate(args);
                        subtrie = subtrace.data;
                        retv = Rc::new(subtrace.retv.unwrap());
                        logp = subtrace.logp;
                    }
                }

                *weight += logp;
                data.insert_internal_node(addr, subtrie);

                retv.as_ref().clone()
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


impl<Args: Clone + 'static,Ret: 'static> GenFn<Args,Trie<(Rc<dyn Any>,f64)>,Ret> for TrieFn<Args,Ret> {
    fn rng(&self) -> ThreadRng { self.rng.clone() }

    fn simulate(&mut self, args: Args) -> Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret> {
        let mut state = TrieFnState::Simulate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
            visitor: AddressVisitor::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Simulate {mut trace, visitor} = state else { unreachable!() };
        trace.set_retv(retv);
        trace
    }

    fn generate(&mut self, args: Args, constraints: Trie<(Rc<dyn Any>,f64)>) -> (Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret>, f64) {
        let mut state = TrieFnState::Generate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
            weight: 0.,
            constraints: constraints.unweighted(),
            visitor: AddressVisitor::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Generate {mut trace, weight, constraints, visitor} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&mut self,
        trace: &mut Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret>,
        args: Args,
        _: GfDiff,
        constraints: Trie<(Rc<dyn Any>,f64)>
    ) -> (Trie<(Rc<dyn Any>,f64)>, f64) {
        let mut state = TrieFnState::Update {
            trace: trace,
            weight: 0.,
            constraints: constraints.unweighted(),
            discard: Trie::new(),
            visitor: AddressVisitor::new()
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Update {mut trace, weight, constraints, discard, visitor} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (Trie::from_unweighted(discard), weight)
    }
}