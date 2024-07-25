use std::sync::Arc;
use std::any::Any;
use rand::rngs::ThreadRng;
use crate::AddrMap;
use crate::modeling::dists::Distribution;
use crate::{Trie, GenFn,ArgDiff, Trace};


pub type DynTrie = Trie<Arc<dyn Any + Send + Sync>>;
pub type DynTrace<Args,Ret> = Trace<Args,DynTrie,Ret>;

impl DynTrie {
    /// Cast the inner `dyn Any` at `addr` into type `V` at runtime.
    pub fn read<V: 'static + Clone>(&self, addr: &str) -> V {
        match self.search(addr) {
            Some(v) => {
                let v_typed = v
                    .ref_inner()
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
        ///
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
    },

    /// State for executing `GenFn::generate` in a `DynGenFn`.
    Generate {
        ///
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
        ///
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
        ///
        diff:ArgDiff,
        ///
        constraints: DynTrie,
        ///
        weight: f64,
        ///
        discard: DynTrie,
        ///
        visitor: AddrMap
    },

    /// State for executing `GenFn::regenerate` in a `DynGenFn`.
    Regenerate {
        ///
        prng: &'a mut ThreadRng,
        ///
        trace: DynTrace<A,T>,
        ///
        diff:ArgDiff,
        ///
        mask: &'a AddrMap,
        ///
        weight: f64,
        ///
        visitor: AddrMap
    }
}


impl<A,T> DynGenFnHandler<'_,A,T> {
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
                    Some(choice) => {
                        debug_assert!(choice.is_leaf());
                        let x = choice
                            .expect_inner(&format!("error: no value found in {addr}"))
                            .downcast::<V>()
                            .expect(&format!("error: downcast failed at {addr}"));
                        let logp = dist.logpdf(x.as_ref(), args);
                        *weight += logp;
                        (x, logp)
                    }
                    None => {
                        let x = dist.random(prng, args.clone());
                        let logp = dist.logpdf(&x, args);
                        (Arc::new(x), logp)
                    }
                };

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
                            *weight -= call.weight();
                            discard.insert(addr, call);
                        };
                        let x = choice
                            .expect_inner(&format!("error: no value found in {addr}"))
                            .downcast::<V>()
                            .expect(&format!("error: downcast failed at {addr}"));
                        let logp = dist.logpdf(x.as_ref(), args);
                        *diff =ArgDiff::Unknown;
                        *weight += logp;
                        (x, logp)
                    }
                    None => {
                        match trace.data.remove(addr) {
                            Some(call) => {
                                match diff {
                                   ArgDiff::NoChange => {
                                        let x = call
                                            .clone()
                                            .expect_inner(&format!("error: no value found in {addr}"))
                                            .downcast::<V>()
                                            .expect(&format!("error: downcast failed at {addr}"));
                                        trace.data.insert(addr, call);
                                        return x.as_ref().clone();
                                    }
                                   ArgDiff::Unknown => {
                                        let prev_logp = call.weight();
                                        let x = call
                                            .clone()
                                            .expect_inner(&format!("error: no value found in {addr}"))
                                            .downcast::<V>()
                                            .expect(&format!("error: downcast failed at {addr}"));
                                        let logp = dist.logpdf(x.as_ref(), args);
                                        *weight += logp - prev_logp;
                                        (x, logp)
                                    }
                                    _ => {
                                        panic!("update:ArgDiff::Extend not supported");
                                    }
                                }
                            }
                            None => {
                                let x = Arc::new(dist.random(prng, args.clone()));
                                let logp = dist.logpdf(x.as_ref(), args);
                                *diff =ArgDiff::Unknown;
                                (x, logp)
                            }
                        }
                    }
                };

                trace.data.witness(addr, x.clone(), logp);
                x.as_ref().clone()
            }

            DynGenFnHandler::Regenerate {
                prng,
                trace,
                diff,
                mask,
                weight,
                visitor
            } => {
                visitor.visit(addr);

                let (x, logp) = match mask.search(addr) {
                    Some(submask) => {
                        debug_assert!(submask.is_leaf());
                        trace.data.remove(addr);  // remove (if has previous)
                        let x = Arc::new(dist.random(prng, args.clone()));
                        let logp = dist.logpdf(x.as_ref(), args);
                        *diff =ArgDiff::Unknown;
                        (x, logp)
                    }
                    None => {
                        match trace.data.remove(addr) {
                            Some(call) => {
                                match diff {
                                   ArgDiff::NoChange => {
                                        let x = call
                                            .clone()
                                            .expect_inner(&format!("error: no value found in {addr}"))
                                            .downcast::<V>()
                                            .expect(&format!("error: downcast failed at {addr}"));
                                        trace.data.insert(addr, call);
                                        return x.as_ref().clone();
                                    }
                                   ArgDiff::Unknown => {
                                        let prev_logp = call.weight();
                                        let x = call
                                            .clone()
                                            .expect_inner(&format!("error: no value found in {addr}"))
                                            .downcast::<V>()
                                            .expect(&format!("error: downcast failed at {addr}"));
                                        let logp = dist.logpdf(x.as_ref(), args);
                                        *weight += logp - prev_logp;
                                        (x, logp)
                                    }
                                    _ => {
                                        panic!("update:ArgDiff::Extend not supported");
                                    }
                                }
                            }
                            None => {
                                let x = Arc::new(dist.random(prng, args.clone()));
                                let logp = dist.logpdf(x.as_ref(), args);
                                *diff =ArgDiff::Unknown;
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
                prng: _,
                trace,
            } => {
                let mut subtrace = gen_fn.simulate(args);
                subtrace.data.replace_inner(Arc::new(subtrace.retv.clone().unwrap()));
                trace.data.insert(addr, subtrace.data);
                subtrace.retv.unwrap()
            }

            DynGenFnHandler::Generate {
                prng: _,
                trace,
                weight,
                constraints,
            } => {
                let (mut sub, retv) = match constraints.remove(addr) {
                    Some(choices) => {
                        debug_assert!(!choices.is_leaf());
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
                trace.data.insert(addr, sub);
                retv.unwrap()
            },

            DynGenFnHandler::Update {
                prng: _,
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
                                debug_assert!(!choices.is_leaf());
                                let logjp = sub.weight();
                                let subtrace = Trace { args: args.clone(), data: sub, retv: None, logjp };
                                let (subtrace, subdiscard, d_weight) = gen_fn.update(subtrace, args, diff.clone(), choices);
                                if !subdiscard.is_empty() {
                                    discard.insert(addr, subdiscard);
                                }
                                *diff =ArgDiff::Unknown;
                                *weight += d_weight;
                                (subtrace.data, subtrace.retv)
                            }
                            None => {
                                let (subtrace, d_weight) = gen_fn.generate(args, choices);
                                *diff =ArgDiff::Unknown;
                                *weight += d_weight;
                                (subtrace.data, subtrace.retv)
                            }
                        }
                    }
                    None => {
                        match trace.data.remove(addr) {
                            Some(sub) => {
                                match diff {
                                   ArgDiff::NoChange => {
                                        let retv = sub.ref_inner().unwrap().downcast_ref::<Y>().unwrap().clone();
                                        trace.data.insert(addr, sub);
                                        return retv;
                                    }
                                   ArgDiff::Unknown => {
                                        let logjp = sub.weight();
                                        let subtrace = Trace { args: args.clone(), data: sub, retv: None, logjp };
                                        let (subtrace, subdiscard, d_weight) = gen_fn.update(subtrace, args,ArgDiff::Unknown, DynTrie::new());
                                        if !(subdiscard.is_empty()) {
                                            discard.insert(addr, subdiscard);
                                        }
                                        *weight += d_weight;
                                        (subtrace.data, subtrace.retv)
                                    }
                                    _ => {
                                        panic!("update:ArgDiff::Extend not supported");
                                    }
                                }
                            }
                            None => {
                                let subtrace = gen_fn.simulate(args);
                                *diff =ArgDiff::Unknown;
                                (subtrace.data, subtrace.retv)
                            }
                        }
                    }
                };

                sub.replace_inner(Arc::new(retv.clone().unwrap()));
                trace.data.insert(addr, sub);
                retv.unwrap()
            }

            DynGenFnHandler::Regenerate {
                prng: _prng,
                trace,
                diff,
                mask,
                weight,
                visitor
            } => {
                visitor.visit(addr);

                let submask = mask.search(addr);

                let (mut sub, retv) = match trace.data.remove(addr) {
                    Some(sub) => {
                        let logjp = sub.weight();
                        match submask {
                            Some(submask) => {
                                let subtrace = Trace { args: args.clone(), data: sub, retv: None, logjp };
                                let (subtrace, d_weight) = gen_fn.regenerate(subtrace, args, diff.clone(), submask);
                                *diff =ArgDiff::Unknown;
                                *weight += d_weight;
                                (subtrace.data, subtrace.retv)
                            }
                            None => {  // submask is absent
                                match diff {
                                   ArgDiff::NoChange => {
                                        let retv = sub.ref_inner().unwrap().downcast_ref::<Y>().unwrap().clone();
                                        trace.data.insert(addr, sub);
                                        return retv;
                                    }
                                   ArgDiff::Unknown => {
                                        let prev_weight = sub.weight();
                                        let (subtrace, new_weight) = gen_fn.generate(args, sub);
                                        *weight += new_weight - prev_weight;
                                        (subtrace.data, subtrace.retv)
                                    }
                                    _ => {
                                        panic!("update:ArgDiff::Extend not supported");
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        let subtrace = gen_fn.simulate(args);
                        *diff =ArgDiff::Unknown;
                        (subtrace.data, subtrace.retv)
                    }
                };

                sub.replace_inner(Arc::new(retv.clone().unwrap()));
                trace.data.insert(addr, sub);
                retv.unwrap()
            }

        }
    }

    /// For all `addr` present in `self.trace.data`, but not present in `self.visitor`, remove `addr` from `self.trace.data` and merge into `self.discard`.
    /// 
    /// Panics if `self` is not the `Self::Update` or `Self::Regenerate` variant.
    pub fn gc(self) -> Self {
        match self {
            Self::Update { prng, trace, diff, constraints, weight, mut discard, visitor } => {
                let schema = trace.data.schema();
                let (data, complement, complement_weight) = trace.data.collect(&schema.complement(&visitor));
                debug_assert!(visitor.all_visited(&data.schema()));  // all unvisited nodes garbage-collected
                discard.merge(complement);
                Self::Update {
                    prng,
                    trace: Trace { args: trace.args, data, retv: trace.retv, logjp: 0. },
                    diff,
                    constraints,
                    weight: weight - complement_weight,
                    discard,
                    visitor
                }
            }
            Self::Regenerate { prng, trace, diff, mask, weight, visitor } => {
                let schema = trace.data.schema();
                let (data, _, _) = trace.data.collect(&schema.complement(&visitor));
                Self::Regenerate {
                    prng,
                    trace: Trace { args: trace.args, data, retv: trace.retv, logjp: 0. },
                    diff,
                    mask,
                    weight,
                    visitor
                }
            }
            _ => { panic!("garbage-collect (gc): called outside of update or regenerate context") }
        }
    }
}


/// Wrapper struct for functions that use the `DynGenFnHandler` DSL (`sample_at` and `trace_at`) and implement the GFI.
pub struct DynGenFn<A,T> {
    /// A stochastic function that takes in a mutable reference to a `DynGenFnHandler<A,T>` and some args `A`, effectfully mutates the state, and produces a value `T`.
    pub func: fn(&mut DynGenFnHandler<A,T>, A) -> T,
}

impl<Args,Ret> DynGenFn<Args,Ret> {
    /// Dynamically construct a `DynGenFn` from a function at run-time.
    pub const fn new(func: fn(&mut DynGenFnHandler<Args,Ret>, Args) -> Ret) -> Self {
        DynGenFn { func }
    }
}

impl<Args: Clone,Ret> GenFn<Args,DynTrie,Ret> for DynGenFn<Args,Ret> {
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

    fn generate(&self, args: Args, mut constraints: DynTrie) -> (DynTrace<Args,Ret>, f64) {
        constraints.take_inner();  // in case constraints came from a proposal
        let mut g = DynGenFnHandler::Generate {
            prng: &mut ThreadRng::default(),
            trace: Trace { args: args.clone(), data: Trie::new(), retv: None, logjp: 0. },
            weight: 0.,
            constraints: constraints,
        };
        let retv = (self.func)(&mut g, args);
        let DynGenFnHandler::Generate {prng: _, mut trace, weight, constraints} = g else { unreachable!() };
        if !constraints.is_empty() {
            println!("residual found:\n{:#?}", constraints);
            panic!("generate error: not all constraints were consumed!");
        }  // else all constraints bound to trace
        trace.logjp = trace.data.weight();
        trace.set_retv(retv);
        (trace, weight)
    }

    fn update(&self,
        trace: DynTrace<Args,Ret>,
        args: Args,
        diff:ArgDiff,
        mut constraints: DynTrie
    ) -> (DynTrace<Args,Ret>, DynTrie, f64) {
        constraints.take_inner();  // in case constraints came from a proposal
        let mut g = DynGenFnHandler::Update {
            prng: &mut ThreadRng::default(),
            trace,
            diff,
            weight: 0.,
            constraints: constraints,
            discard: Trie::new(),
            visitor: AddrMap::new()
        };
        let retv = (self.func)(&mut g, args);
        let g = g.gc();  // subtract weight of complement and add complement to discard
        let DynGenFnHandler::Update {prng: _, mut trace, diff: _diff, weight, constraints, discard, visitor: _visitor} = g else { unreachable!() };
        if !constraints.is_empty() {
            println!("residual found:\n{:#?}", constraints);
            panic!("update error: not all constraints were consumed!");
        }  // else all constraints bound to trace
        trace.logjp = trace.data.weight();
        trace.set_retv(retv);
        (trace, discard, weight)
    }

    fn regenerate(&self,
        trace: DynTrace<Args,Ret>,
        args: Args,
        diff:ArgDiff,
        mask: &AddrMap
    ) -> (DynTrace<Args,Ret>, f64) {
        let mut g = DynGenFnHandler::Regenerate {
            prng: &mut ThreadRng::default(),
            mask: if mask.is_leaf() { &trace.data.schema() } else { mask },
            trace,
            diff,
            weight: 0.,
            visitor: AddrMap::new()
        };
        let retv = (self.func)(&mut g, args);
        let g = g.gc();
        let DynGenFnHandler::Regenerate {prng: _, mut trace, diff: _diff, mask: _mask, weight, visitor: _visitor} = g else { unreachable!() };
        trace.logjp = trace.data.weight();
        trace.set_retv(retv);
        (trace, weight)
    }

}