use std::sync::Arc;
use std::any::Any;
use rand::rngs::ThreadRng;
use crate::modeling::dists::Distribution;
use crate::{Trie, AddrTrie, GenFn, GfDiff, Trace};


pub type DynTrie = Trie<Arc<dyn Any + Send + Sync>>;
pub type DynTrace<Args,Ret> = Trace<Args,DynTrie,Ret>;

impl DynTrie {
    /// Cast the inner `dyn Any` at `addr` into type `V` at runtime.
    pub fn read<V: 'static + Clone>(&self, addr: &str) -> V {
        match self.search(addr) {
            Some(v) => {
                let v_typed = v
                    .value_ref()
                    .unwrap()
                    .downcast_ref::<V>();
                match v_typed {
                    Some(v) => { v.clone() }
                    None => {
                        panic!("read: failed when downcasting type at address \"{}\"", addr);
                    }
                }
            }
            None => {
                panic!("read: failed when searching empty address \"{}\"", addr);
            }
        }
    }
}

/// Incremental computational state of a `trace` during the execution of the different `GenFn` methods with a `DynGenFn`.
pub enum DynGenFnHandler<'a,A,T> {
    /// State for executing `GenFn::simulate` in a `DynGenFn`.
    Simulate {
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
    },

    /// State for executing `GenFn::generate` in a `DynGenFn`.
    Generate {
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
        ///
        weight: f64,
        ///
        constraints: DynTrie,
    },

    /// State for executing `GenFn::update` in a `DynGenFn`.
    Update {
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
        ///
        diff: GfDiff,
        ///
        constraints: DynTrie,
        ///
        weight: f64,
        ///
        discard: DynTrie,
        ///
        visitor: AddrTrie
    }
}


impl<A: 'static,T: 'static> DynGenFnHandler<'_,A,T> {
    /// Sample a random value from a distribution and insert it into the `self.trace.data` trie as a weighted leaf node.
    /// 
    /// Return a clone of the sampled value.
    pub fn sample_at<
        V: Clone + Send + Sync + 'static,
        W: Clone + 'static
    >(&mut self, dist: &impl Distribution<V,W>, args: W, addr: &str) -> V {
        match self {
            DynGenFnHandler::Simulate {
                prng,
                trace,
            } => {
                let x = dist.random(prng, args.clone());
                let logp = dist.logpdf(&x, args);
                trace.data.witness(addr, Arc::new(x.clone()), logp);
                x
            }

            DynGenFnHandler::Generate {
                prng,
                trace,
                weight,
                constraints,
            } => {
                let (x, logp) = match constraints.remove(addr) {
                    // if Some, cast to type V, calculate change to trace.logp (and add to weight)
                    Some(choice) => {
                        debug_assert!(choice.is_leaf());
                        let x = choice.unwrap_inner_unchecked().downcast::<V>().ok().unwrap();
                        let logp = dist.logpdf(x.as_ref(), args);
                        *weight += logp;
                        (x, logp)
                    }
                    // if None, sample a value and calculate change to trace.logp
                    None => {
                        let x = dist.random(prng, args.clone());
                        let logp = dist.logpdf(&x, args);
                        (Arc::new(x), logp)
                    }
                };

                // mutate trace with sampled leaf, increment total trace.logp, and insert in trie.
                trace.data.witness(addr, x.clone(), logp);
                x.as_ref().clone()
            }

            DynGenFnHandler::Update {
                prng,
                trace,
                diff,
                constraints,
                weight,
                discard,
                visitor
            } => {
                visitor.visit(addr);

                let (x, logp) = match constraints.remove(addr) {
                    Some(choice) => {
                        debug_assert!(choice.is_leaf());
                        if let Some(call) = trace.data.remove(addr) {
                            debug_assert!(call.is_leaf());
                            discard.insert(addr, call);
                        };
                        let x = choice.unwrap_inner_unchecked().downcast::<V>().ok().unwrap();
                        let logp = dist.logpdf(x.as_ref(), args);
                        *weight += logp;
                        (x, logp)
                    }
                    None => {
                        match trace.data.remove(addr) {
                            Some(call) => {
                                match diff {
                                    GfDiff::NoChange => {
                                        let x = call.clone().unwrap_inner_unchecked().downcast::<V>().ok().unwrap();
                                        trace.data.insert(addr, call);
                                        return x.as_ref().clone();
                                    }
                                    GfDiff::Unknown => {
                                        let prev_logp = call.weight();
                                        let x = call.clone().unwrap_inner_unchecked().downcast::<V>().ok().unwrap();
                                        let logp = dist.logpdf(x.as_ref(), args);
                                        *weight += logp - prev_logp;
                                        (x, logp)
                                    }
                                    _ => {
                                        panic!("update: GfDiff::Extend not supported");
                                    }
                                }
                            }
                            None => {
                                let x = Arc::new(dist.random(prng, args.clone()));
                                let logp = dist.logpdf(x.as_ref(), args);
                                (x, logp)
                            }
                        }
                    }
                };

                trace.data.witness(addr, x.clone(), logp);
                x.as_ref().clone()
            }
        }
    }

    /// Recursively sample a trace from another `gen_fn`.
    /// 
    /// Insert its `subtrace.data` trie as a weighted internal node of the current `trace.data` trie.
    /// Insert its `retv` as a (zero-weighted) internal node of the current `trace.data` trie.
    /// 
    /// Return a clone of the `retv`.
    pub fn trace_at<
        X: Clone + 'static,
        Y: Clone + Send + Sync + 'static
    >(&mut self, gen_fn: &impl GenFn<X,DynTrie,Y>, args: X, addr: &str) -> Y {
        match self {
            DynGenFnHandler::Simulate {
                prng,
                trace,
            } => {
                let subtrace = gen_fn.simulate(args);
                assert!(trace.data.insert(addr, subtrace.data).is_none());
                subtrace.retv.unwrap()
            }

            DynGenFnHandler::Generate {
                prng,
                trace,
                weight,
                constraints,
            } => {
                let (mut sub, retv) = match constraints.remove(addr) {
                    Some(choices) => {
                        let (subtrace, d_weight) = gen_fn.generate(args, choices);
                        *weight += d_weight;
                        (subtrace.data, subtrace.retv)
                    }
                    None => {
                        let subtrace = gen_fn.simulate(args);
                        (subtrace.data, subtrace.retv)
                    }
                };
                sub.replace_inner(Arc::new(retv.clone().unwrap()));
                assert!(trace.data.insert(addr, sub).is_none());
                retv.unwrap()
            },

            DynGenFnHandler::Update {
                prng,
                trace,
                diff,
                constraints,
                weight,
                discard,
                visitor
            } => {
                visitor.visit(addr);

                let (mut sub, retv) = match constraints.remove(addr) {
                    Some(choices) => {
                        match trace.data.remove(addr) {
                            Some(sub) => {
                                let logjp = sub.weight();
                                let subtrace = Trace { args: args.clone(), data: sub, retv: None, logjp };
                                let (subtrace, subdiscard, d_weight) = gen_fn.update(subtrace, args, GfDiff::Unknown, choices);
                                if !subdiscard.is_empty() {
                                    discard.insert(addr, subdiscard);
                                }
                                *weight += d_weight;
                                (subtrace.data, subtrace.retv)
                            }
                            None => {
                                let (subtrace, d_weight) = gen_fn.generate(args, choices);
                                *weight += d_weight;
                                (subtrace.data, subtrace.retv)
                            }
                        }
                    }
                    None => {
                        match trace.data.remove(addr) {
                            Some(sub) => {
                                match diff {
                                    GfDiff::NoChange => {
                                        let retv = sub.value_ref().unwrap().downcast_ref::<Y>().unwrap().clone();
                                        assert!(trace.data.insert(addr, sub).is_none());
                                        return retv;
                                    }
                                    GfDiff::Unknown => {
                                        let logjp = sub.weight();
                                        let subtrace = Trace { args: args.clone(), data: sub.clone(), retv: None, logjp };
                                        let (subtrace, subdiscard, d_weight) = gen_fn.update(subtrace, args, GfDiff::Unknown, DynTrie::new());
                                        if !(subdiscard.is_empty()) {
                                            discard.insert(addr, subdiscard);
                                        }
                                        *weight += d_weight;
                                        (subtrace.data, subtrace.retv)
                                    }
                                    _ => {
                                        panic!("update: GfDiff::Extend not supported");
                                    }
                                }
                            }
                            None => {
                                let subtrace = gen_fn.simulate(args);
                                (subtrace.data, subtrace.retv)
                            }
                        }
                    }
                };
                sub.replace_inner(Arc::new(retv.clone().unwrap()));
                assert!(trace.data.insert(addr, sub).is_none());
                retv.unwrap()
            }
        }
    }

    pub fn collect_unvisited(
        mut trie: DynTrie,
        unvisited: &AddrTrie,
    ) -> (DynTrie,DynTrie) {
        let mut garbage = Trie::new();
        // todo: profile this and make more efficient (eg. with Merkle trees)
        if &AddrTrie::schema(&trie) == unvisited {
            return (garbage, trie);
        } else if !unvisited.is_empty() {
            for (addr, subunvisited) in unvisited.iter() {
                let Some(sub) = trie.remove(addr) else { unreachable!() };
                if subunvisited.is_leaf() {
                    garbage.insert(addr, sub);
                } else {
                    let (sub, subgarbage) = Self::collect_unvisited(sub, subunvisited);
                    if !sub.is_empty() {
                        trie.insert(addr, sub);
                    }
                    if !subgarbage.is_empty() {
                        garbage.insert(addr, subgarbage);
                    }
                }
            }
        }
        (trie, garbage)
    }

    /// For all `addr` present in `self.trace.data`, but not present in `self.visitor`, remove `addr` from `self.trace.data` and merge into `self.discard`.
    /// 
    /// Panics if `self` is not the `Self::Update` variant.
    pub fn gc(self) -> Self {
        if let Self::Update { prng, trace, diff, constraints, weight, mut discard, visitor } = self {
            let unvisited = visitor.get_unvisited(&trace.data);
            let (data, garbage) = Self::collect_unvisited(trace.data, &unvisited);
            assert!(visitor.all_visited(&data));  // all unvisited nodes garbage-collected
            let data_weight = data.weight();
            discard.merge(garbage);
            let discard_weight = discard.weight();
            Self::Update {
                prng,
                trace: Trace { args: trace.args, data, retv: trace.retv, logjp: data_weight - discard_weight },
                diff,
                constraints,
                weight: weight - discard_weight,
                discard,
                visitor
            }
        } else { panic!("garbage-collect (gc) called outside of update context") }
    }
}


/// Wrapper struct for functions that use the `DynGenFnHandler` DSL (`sample_at` and `trace_at`) and automatically implement the GFI.
pub struct DynGenFn<A,T> {
    /// A random function that takes in a mutable reference to a `DynGenFnHandler<A,T>` and some args `A`, effectfully mutates the state, and produces a value `T`.
    pub func: fn(&mut DynGenFnHandler<A,T>, A) -> T,
}

impl<Args,Ret> DynGenFn<Args,Ret> {
    /// Dynamically construct a `DynGenFn` from a function at run-time.
    pub const fn new(func: fn(&mut DynGenFnHandler<Args,Ret>, Args) -> Ret) -> Self {
        DynGenFn { func }
    }
}


impl<Args: Clone + 'static,Ret: 'static> GenFn<Args,DynTrie,Ret> for DynGenFn<Args,Ret> {
    fn simulate(&self, args: Args) -> DynTrace<Args,Ret> {
        let mut g = DynGenFnHandler::Simulate {
            prng: &mut ThreadRng::default(),
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logjp: 0. },
        };
        let retv = (self.func)(&mut g, args);
        let DynGenFnHandler::Simulate {prng: _, mut trace} = g else { unreachable!() };
        trace.set_retv(retv);
        trace.logjp = trace.data.weight();
        trace
    }

    fn generate(&self, args: Args, constraints: DynTrie) -> (DynTrace<Args,Ret>, f64) {
        let mut g = DynGenFnHandler::Generate {
            prng: &mut ThreadRng::default(),
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logjp: 0. },
            weight: 0.,
            constraints: constraints,
        };
        let retv = (self.func)(&mut g, args);
        let DynGenFnHandler::Generate {prng: _, mut trace, weight, constraints} = g else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.logjp = trace.data.weight();
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&self,
        trace: DynTrace<Args,Ret>,
        args: Args,
        diff: GfDiff,
        constraints: DynTrie
    ) -> (DynTrace<Args,Ret>, DynTrie, f64) {
        let mut g = DynGenFnHandler::Update {
            prng: &mut ThreadRng::default(),
            trace,
            diff: if diff == GfDiff::NoChange && constraints.is_empty() { diff } else { GfDiff::Unknown },
            weight: 0.,
            constraints: constraints,
            discard: Trie::new(),
            visitor: AddrTrie::new()
        };
        let retv = (self.func)(&mut g, args);
        let g = g.gc();  // add unvisited to discard
        let DynGenFnHandler::Update {prng: _, mut trace, diff: _diff, weight, constraints, discard, visitor: _visitor} = g else { unreachable!() };
        assert!(constraints.is_empty());  // all constraints bound to trace
        trace.logjp = trace.data.weight();
        trace.set_retv(retv);
        (trace, discard, weight)
    }
}