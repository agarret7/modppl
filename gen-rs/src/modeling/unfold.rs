use crate::{DynGenFn, DynGenFnHandler, DynTrie, GenFn, GfDiff, Trace};
use rand::rngs::ThreadRng;


/// Combinator struct for kernels that use the `DynGenFnHandler` DSL (`sample_at` and `trace_at`) and automatically implement the GFI.
/// Supports memory-efficient extension via the `GfDiff::Extend` flag (eg. as passed during a `ParticleSystem::step`).
pub struct Unfold<State> {
    /// A random kernel that takes in a mutable reference to a `DynGenFnHandler<A,final_t>` and some `State`, effectfully mutates it, and produces a new `State`.
    pub kernel: DynGenFn<(i64,State),State>
}

impl<State> Unfold<State> {
    /// Dynamically construct an `Unfold` from a kernel function at run-time.
    pub fn new(kernel: DynGenFn<(i64,State),State>) -> Self {
        Unfold { kernel }
    }
}

use crate::ParticleSystem;
pub type Particles<State> = ParticleSystem<State,Vec<DynTrie>,Vec<State>,Unfold<State>>;


impl<State: Clone> GenFn<(i64,State),Vec<DynTrie>,Vec<State>> for Unfold<State> {
    fn simulate(&self, final_t_and_args: (i64, State)) -> Trace<(i64,State),Vec<DynTrie>,Vec<State>> {
        let (final_t, mut state) = final_t_and_args;
        assert!(final_t >= 1);
        let mut vec_trace = Trace { args: (final_t, state.clone()), data: vec![], retv: Some(vec![]), logjp: 0. };
        for t in 0..final_t {
            let mut g = DynGenFnHandler::Simulate {
                prng: &mut ThreadRng::default(),
                trace: Trace { args: (t as i64, state.clone()), data: DynTrie::new(), retv: None, logjp: 0. },
            };
            state = (self.kernel.func)(&mut g, (t as i64, state.clone()));
            let DynGenFnHandler::Simulate {prng: _, trace} = g else { unreachable!() };
            vec_trace.retv.as_mut().unwrap().push(state.clone());
            vec_trace.data.push(trace.data);
            vec_trace.logjp += trace.logjp;
        }
        vec_trace
    }

    fn generate(&self, final_t_and_args: (i64, State), vec_constraints: Vec<DynTrie>) 
        -> (Trace<(i64,State),Vec<DynTrie>,Vec<State>>, f64)
    {
        let (final_t, mut state) = final_t_and_args;
        assert!(final_t >= 1);
        let mut vec_trace = Trace { args: (final_t, state.clone()), data: vec![], retv: Some(vec![]), logjp: 0. };
        let mut gen_weight = 0.;
        for (t,constraints) in vec_constraints.into_iter().enumerate() {
            let mut g = DynGenFnHandler::Generate {
                prng: &mut ThreadRng::default(),
                trace: Trace { args: (t as i64, state.clone()), data: DynTrie::new(), retv: None, logjp: 0. },
                weight: 0.,
                constraints
            };
            state = (self.kernel.func)(&mut g, (t as i64, state.clone()));
            let DynGenFnHandler::Generate {prng: _, trace, weight, constraints} = g else { unreachable!() };
            assert!(constraints.is_empty());
            vec_trace.retv.as_mut().unwrap().push(state.clone());
            vec_trace.data.push(trace.data);
            vec_trace.logjp += trace.logjp;
            gen_weight += weight;
        }
        (vec_trace, gen_weight)
    }

    fn update(&self,
        mut vec_trace: Trace<(i64,State),Vec<DynTrie>,Vec<State>>,
        final_t_and_args: (i64, State),
        diff: GfDiff,
        vec_constraints: Vec<DynTrie>
    ) -> (Trace<(i64,State),Vec<DynTrie>,Vec<State>>, Vec<DynTrie>, f64) {
        let (final_t, _) = final_t_and_args;
        assert!(final_t >= 1);
        let prev_t = vec_trace.args.0;
        assert!(final_t - prev_t == vec_constraints.len() as i64);
        let mut state = vec_trace.retv.as_ref().unwrap().last().unwrap().clone();
        let mut update_weight = 0.;
        match diff {
            GfDiff::Extend => {
                for (t,constraints) in vec_constraints.into_iter().enumerate() {
                    let mut g = DynGenFnHandler::Generate {
                        prng: &mut ThreadRng::default(),
                        trace: Trace { args: (prev_t + (t as i64), state.clone()), data: DynTrie::new(), retv: None, logjp: 0. },
                        weight: 0.,
                        constraints
                    };
                    state = (self.kernel.func)(&mut g, (prev_t + (t as i64), state.clone()));
                    let DynGenFnHandler::Generate {prng: _, trace, weight, constraints} = g else { unreachable!() };
                    assert!(constraints.is_empty());
                    vec_trace.args.0 += 1;
                    vec_trace.retv.as_mut().unwrap().push(state.clone());
                    vec_trace.data.push(trace.data);
                    vec_trace.logjp += trace.logjp;
                    update_weight += weight;
                }
            },
            _ => { panic!("Can't handle GF change type: {:?}", diff) },
        }
        (vec_trace, (prev_t..final_t).map(|_| DynTrie::new()).collect::<_>(), update_weight)
    }
}