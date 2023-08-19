use std::rc::Rc;
use std::any::Any;
use crate::modeling::dists::Distribution;
use crate::{GLOBAL_RNG, Trie, GenFn, GfDiff, Trace};


/// Incremental computational state of a `trace` during the execution of the different `GenFn` methods with a `TrieFn`.
pub enum TrieFnState<A,T> {
    /// State for executing `GenFn::simulate` in a `TrieFn`.
    Simulate {
        ///
        trace: Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
    },

    /// State for executing `GenFn::generate` in a `TrieFn`.
    Generate {
        ///
        trace: Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
        ///
        weight: f64,
        ///
        constraints: Trie<Rc<dyn Any>>,
    },

    /// State for executing `GenFn::update` in a `TrieFn`.
    Update {
        ///
        trace: Trace<A,Trie<(Rc<dyn Any>,f64)>,T>,
        ///
        constraints: Trie<Rc<dyn Any>>,
        ///
        weight: f64,
        ///
        discard: Trie<Rc<dyn Any>>,
        ///
        visitor: AddrTrie
    }
}

/// Trie for hierarchical address schemas (without values).
pub type AddrTrie = Trie<()>;

impl AddrTrie {
    /// Add an address to the `AddrTrie`.
    pub fn visit(&mut self, addr: &str) {
        self.insert_leaf_node(addr, ());
    }

    /// Return `true` if every `addr` in `data` is also present in `self`.
    pub fn all_visited<T>(&self, data: &Trie<T>) -> bool {
        let mut allvisited = true;
        for (addr, _) in data.leaf_iter() {
            allvisited = allvisited && self.has_leaf_node(&addr);
        }
        for (addr, inode) in data.internal_iter() {
            if !self.has_leaf_node(&addr) {
                let subvisited = self.get_internal_node(&addr).unwrap();
                allvisited = allvisited && subvisited.all_visited(inode)
            }
        }
        allvisited
    }

    /// Return the `AddrTrie` that contains all addresses present in `data`, but not present in `self`.
    pub fn get_unvisited<V>(&self, data: &Trie<V>) -> Self {
        let mut unvisited = Trie::new();
        for (addr, _) in data.leaf_iter() {
            if !self.has_leaf_node(&addr) {
                unvisited.insert_leaf_node(&addr, ());
            }
        }
        for (addr, inode) in data.internal_iter() {
            if !self.has_leaf_node(&addr) {
                let subvisited = self.get_internal_node(&addr).unwrap();
                let sub_unvisited = subvisited.get_unvisited(inode);
                unvisited.insert_internal_node(&addr, sub_unvisited);
            }
        }
        unvisited
    }

    /// Return the unique `AddrTrie` that contains an `addr` if and only if `data` contains that `addr`.
    pub fn schema<V>(data: &Trie<V>) -> Self {
        let mut visitor = Trie::new();
        for (addr, _) in data.leaf_iter() {
            visitor.insert_leaf_node(addr, ());
        }
        for (addr, inode) in data.internal_iter() {
            visitor.insert_internal_node(addr, Self::schema(inode));
        }
        visitor
    }
}

impl<A: 'static,T: 'static> TrieFnState<A,T> {
    /// Sample a random value from a distribution and insert it into the `self.trace.data` trie as a weighted leaf node.
    /// 
    /// Return the cloned sampled value.
    pub fn sample_at<
        V: Clone + 'static,
        W: Clone + 'static
    >(&mut self, dist: &impl Distribution<V,W>, args: W, addr: &str) -> V {
        match self {
            TrieFnState::Simulate {
                trace,
            } => {
                let x = GLOBAL_RNG.with_borrow_mut(|rng| {
                    dist.random(rng, args.clone())
                });
                let logp = dist.logpdf(&x, args);
                let data = &mut trace.data;
                data.insert_leaf_node(addr, (Rc::new(x.clone()), logp));
                trace.logp += logp;
                x
            }

            TrieFnState::Generate {
                trace,
                weight,
                constraints,
            } => {
                // check if there are constraints
                let (x, logp) = match constraints.remove_leaf_node(addr) {
                    // if None, sample a value and calculate change to trace.logp
                    None => {
                        let x = GLOBAL_RNG.with_borrow_mut(|rng| {
                            dist.random(rng, args.clone())
                        });
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
                let data = &mut trace.data;
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

                let data = &mut trace.data;
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
                        x = Rc::new(GLOBAL_RNG.with_borrow_mut(|rng| {
                            dist.random(rng, args.clone())
                        }));
                    }
                }

                let logp = dist.logpdf(x.as_ref(), args);
                let d_logp = logp - prev_logp;
                *weight += d_logp;

                data.insert_leaf_node(addr, (x.clone(), logp));
                trace.logp += d_logp;

                x.as_ref().clone()
            }
        }
    }

    /// Recursively sample a trace from another `gen_fn`.
    /// 
    /// Insert its `data` trie as a weighted internal node of the current `trace` data trie.
    /// Insert its `retv` as a (zero-weighted) leaf node of the current `trace` data trie.
    /// 
    /// Return the cloned `retv`.
    pub fn trace_at<
        X: Clone + 'static,
        Y: Clone + 'static,
    >(&mut self, gen_fn: &impl GenFn<X,Trie<(Rc<dyn Any>,f64)>,Y>, args: X, addr: &str) -> Y {
        match self {
            TrieFnState::Simulate {
                trace,
            } => {
                let subtrace = gen_fn.simulate(args);

                let data = &mut trace.data;
                data.insert_internal_node(addr, subtrace.data);

                let retv = subtrace.retv.unwrap();
                data.insert_leaf_node(addr, (Rc::new(retv.clone()), 0.));
                trace.logp += subtrace.logp;

                retv
            }

            TrieFnState::Generate {
                trace,
                weight,
                constraints,
            } => {
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

                let data = &mut trace.data;
                data.insert_internal_node(addr, subtrace.data);

                let retv = subtrace.retv.unwrap().clone();
                data.insert_leaf_node(addr, (Rc::new(retv.clone()), 0.));
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

                let data = &mut trace.data;
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
                        // in case constraints came from a proposal
                        constraints.remove_leaf_node(addr);
                        let prev_logp = prev_subtrie.sum();
                        // note: the args in the subtrace are technically incorrect (they should be from
                        // the previous call) and update only works because we completely disregard them.
                        let subtrace = Trace { args: args.clone(), data: prev_subtrie, retv: None, logp: prev_logp };
                        let (subtrace, subdiscard, new_weight) = gen_fn.update(subtrace, args, GfDiff::Unknown, subconstraints);
                        discard.insert_internal_node(addr, subdiscard.into_unweighted());
                        subtrie = subtrace.data;
                        retv = Rc::new(subtrace.retv.unwrap());
                        logp = new_weight;
                    } else {
                        subtrie = prev_subtrie;
                        retv = data.remove_leaf_node(addr).unwrap().0.downcast::<Y>().ok().unwrap();
                    }
                } else {
                    if constrained {
                        let subconstraints = Trie::from_unweighted(constraints.remove_internal_node(addr).unwrap());
                        let (subtrace, new_weight) = gen_fn.generate(args, subconstraints);
                        subtrie = subtrace.data;
                        retv = Rc::new(subtrace.retv.unwrap());
                        logp = new_weight;
                    } else {
                        let subtrace = gen_fn.simulate(args);
                        subtrie = subtrace.data;
                        retv = Rc::new(subtrace.retv.unwrap());
                        logp = subtrace.logp;
                    }
                }

                *weight += logp;
                data.insert_internal_node(addr, subtrie);
                data.insert_leaf_node(addr, (retv.clone(), 0.));
                trace.logp += logp;

                retv.as_ref().clone()
            }
        }
    }

    fn _gc(
        mut trie: Trie<(Rc<dyn Any>,f64)>,
        unvisited: &AddrTrie,
    ) -> (Trie<(Rc<dyn Any>,f64)>,Trie<Rc<dyn Any>>,f64) {
        let mut garbage = Trie::new();
        let mut garbage_weight = 0.;
        // todo: profile this and make more efficient (eg. with Merkle trees)
        if &AddrTrie::schema(&trie) == unvisited {
            garbage_weight = trie.sum();
            garbage = trie.into_unweighted();
            trie = Trie::new();
        } else if !unvisited.is_empty() {
            for (addr, _) in unvisited.leaf_iter() {
                let Some((value, logp)) = trie.remove_leaf_node(addr) else { unreachable!() };
                garbage.insert_leaf_node(addr, value);
                garbage_weight += logp;
            }
            for (addr, sub_unvisited) in unvisited.internal_iter() {
                let Some(subtrie) = trie.remove_internal_node(addr) else { unreachable!() };
                let (subtrie, subgarbage, logp) = Self::_gc(subtrie, sub_unvisited);
                if !subtrie.is_empty() {
                    trie.insert_internal_node(addr, subtrie);
                }
                if !subgarbage.is_empty() {
                    garbage.insert_internal_node(addr, subgarbage);
                }
                garbage_weight += logp;
            }
        }
        (trie, garbage, garbage_weight)
    }

    /// For all `addr` present in `self.trace.data`, but not present in `self.visitor`, remove `addr` from `self.trace.data` and merge into `self.discard`.
    /// 
    /// Panics if `self` is not the `Self::Update` variant.
    pub fn gc(self) -> Self {
        if let Self::Update { trace, constraints, weight, discard, visitor } = self {
            let unvisited = visitor.get_unvisited(&trace.data);
            let (data, garbage, garbage_weight) = Self::_gc(trace.data, &unvisited);
            assert!(visitor.all_visited(&data));  // all unvisited nodes garbage-collected
            Self::Update {
                trace: Trace { args: trace.args, data, retv: trace.retv, logp: trace.logp },
                constraints,
                weight: weight - garbage_weight,
                discard: discard.merge(garbage),
                visitor
            }
        } else { panic!("garbage-collect (gc) called outside of update context") }
    }
}


/// Wrapper struct for functions that can use the TrieFn DSL (`sample_at` and `trace_at`) and automatically implement the GFI.
pub struct TrieFn<A,T> {
    /// A random function that takes in a mutable reference to a `TrieFnState<A,T>` and some args `A`, effectfully mutates the state, and produces a value `T`.
    pub func: fn(&mut TrieFnState<A,T>, A) -> T,
}

impl<Args,Ret> TrieFn<Args,Ret>{
    /// Dynamically construct a `TrieFn` from a function at run-time.
    pub fn new(func: fn(&mut TrieFnState<Args,Ret>, Args) -> Ret) -> Self {
        TrieFn { func }
    }
}


impl<Args: Clone + 'static,Ret: 'static> GenFn<Args,Trie<(Rc<dyn Any>,f64)>,Ret> for TrieFn<Args,Ret> {
    fn simulate(&self, args: Args) -> Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret> {
        let mut state = TrieFnState::Simulate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Simulate {mut trace} = state else { unreachable!() };
        trace.set_retv(retv);
        trace
    }

    fn generate(&self, args: Args, constraints: Trie<(Rc<dyn Any>,f64)>) -> (Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret>, f64) {
        let mut state = TrieFnState::Generate {
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logp: 0. },
            weight: 0.,
            constraints: constraints.into_unweighted(),
        };
        let retv = (self.func)(&mut state, args);
        let TrieFnState::Generate {mut trace, weight, constraints} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&self,
        trace: Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret>,
        args: Args,
        _: GfDiff,
        constraints: Trie<(Rc<dyn Any>,f64)>
    ) -> (Trace<Args,Trie<(Rc<dyn Any>,f64)>,Ret>, Trie<(Rc<dyn Any>,f64)>, f64) {
        let mut state = TrieFnState::Update {
            trace,
            weight: 0.,
            constraints: constraints.into_unweighted(),
            discard: Trie::new(),
            visitor: AddrTrie::new()
        };
        let retv = (self.func)(&mut state, args);
        let state = state.gc();  // add unvisited to discard
        let TrieFnState::Update {mut trace, weight, constraints, discard, visitor: _visitor} = state else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.set_retv(retv);
        (trace, Trie::from_unweighted(discard), weight)
    }
}