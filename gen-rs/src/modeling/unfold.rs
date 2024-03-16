use crate::{Trace,GenFn,GfDiff,DynTrie,DynGenFnHandler};
use rand::rngs::ThreadRng;
use std::any::Any;
use std::rc::Rc;


/// Combinator struct for kernels that use the `DynGenFnHandler` DSL (`sample_at` and `trace_at`) and automatically implement the GFI.
/// Supports memory-efficient extension via the `GfDiff::Extend` flag (eg. as passed during a `ParticleSystem::step`).
pub struct Unfold<State> {
    /// A random kernel that takes in a mutable reference to a `DynGenFnHandler<A,T>` and some `State`, effectfully mutates it, and produces a new `State`.
    pub kernel: fn(&mut DynGenFnHandler<(i64,State),State>, (i64,State)) -> State
}

impl<State> Unfold<State> {
    /// Dynamically construct an `Unfold` from a kernel at run-time.
    fn new(kernel: fn(&mut DynGenFnHandler<(i64,State),State>, (i64,State)) -> State) -> Self {
        Unfold { kernel }
    }
}


impl<State: Clone> GenFn<(i64,State),Vec<DynTrie>,Vec<State>> for Unfold<State> {
    fn simulate(&self, T_and_args: (i64, State)) -> Trace<(i64,State),Vec<DynTrie>,Vec<State>> {
        let (T, mut state) = T_and_args;
        assert!(T >= 1);
        let mut vec_trace = Trace { args: (T, state.clone()), data: vec![], retv: Some(vec![]), logp: 0. };
        for t in 0..T {
            let mut g = DynGenFnHandler::Simulate {
                prng: &mut ThreadRng::default(),
                trace: Trace { args: (t as i64, state.clone()), data: DynTrie::new(), retv: None, logp: 0. },
            };
            state = (self.kernel)(&mut g, (t as i64, state.clone()));
            let DynGenFnHandler::Simulate {prng: _, mut trace} = g else { unreachable!() };
            vec_trace.retv.as_mut().unwrap().push(state.clone());
            vec_trace.data.push(trace.data);
            vec_trace.logp += trace.logp;
        }
        vec_trace
    }

    fn generate(&self, T_and_args: (i64, State), vec_constraints: Vec<DynTrie>) 
        -> (Trace<(i64,State),Vec<DynTrie>,Vec<State>>, f64)
    {
        let (T, mut state) = T_and_args;
        assert!(T >= 1);
        let mut vec_trace = Trace { args: (T, state.clone()), data: vec![], retv: Some(vec![]), logp: 0. };
        let mut gen_weight = 0.;
        for (t,constraints) in vec_constraints.into_iter().enumerate() {
            let mut g = DynGenFnHandler::Generate {
                prng: &mut ThreadRng::default(),
                trace: Trace { args: (t as i64, state.clone()), data: DynTrie::new(), retv: None, logp: 0. },
                weight: 0.,
                constraints
            };
            state = (self.kernel)(&mut g, (t as i64, state.clone()));
            let DynGenFnHandler::Generate {prng: _, mut trace, weight, constraints} = g else { unreachable!() };
            assert!(constraints.is_empty());
            vec_trace.retv.as_mut().unwrap().push(state.clone());
            vec_trace.data.push(trace.data);
            vec_trace.logp += trace.logp;
            gen_weight += weight;
        }
        (vec_trace, gen_weight)
    }

    fn update(&self,
        mut vec_trace: Trace<(i64,State),Vec<DynTrie>,Vec<State>>,
        T_and_args: (i64, State),
        diff: GfDiff,
        vec_constraints: Vec<DynTrie>
    ) -> (Trace<(i64,State),Vec<DynTrie>,Vec<State>>, Vec<DynTrie>, f64) {
        let (T, _) = T_and_args;
        assert!(T >= 1);
        let prev_T = vec_trace.args.0;
        assert!(T - prev_T == vec_constraints.len() as i64);
        let mut state = vec_trace.retv.as_ref().unwrap().last().unwrap().clone();
        let mut update_weight = 0.;
        match diff {
            GfDiff::Extend => {
                for (t,constraints) in vec_constraints.into_iter().enumerate() {
                    let mut g = DynGenFnHandler::Generate {
                        prng: &mut ThreadRng::default(),
                        trace: Trace { args: (prev_T + (t as i64), state.clone()), data: DynTrie::new(), retv: None, logp: 0. },
                        weight: 0.,
                        constraints
                    };
                    state = (self.kernel)(&mut g, (prev_T + (t as i64), state.clone()));
                    let DynGenFnHandler::Generate {prng: _, mut trace, weight, constraints} = g else { unreachable!() };
                    assert!(constraints.is_empty());
                    vec_trace.args.0 += 1;
                    vec_trace.retv.as_mut().unwrap().push(state.clone());
                    vec_trace.data.push(trace.data);
                    vec_trace.logp += trace.logp;
                    update_weight += weight;
                }
            },
            _ => { panic!("Can't handle GF change type: {:?}", diff) },
        }
        (vec_trace, (prev_T..T).map(|_| DynTrie::new()).collect::<_>(), update_weight)
    }
}