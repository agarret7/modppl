use std::any::Any;
use std::rc::Rc;
use rand::rngs::ThreadRng;
use nalgebra::{DVector,DMatrix};

use super::{ParamStore,HMMTrace};
use gen_rs::{
    Trace,GenerativeFunction,ChoiceBuffer,ChoiceHashMap,GfDiff,
    modeling::dists::{Distribution,categorical}
};


pub struct HMMParams {
    prior: DVector<f64>,
    emission_matrix: DMatrix<f64>,
    transition_matrix: DMatrix<f64>
}

impl HMMParams {
    pub fn new(
        prior: DVector<f64>,
        emission_matrix: DMatrix<f64>,
        transition_matrix: DMatrix<f64>
    ) -> Self {
        HMMParams { prior, emission_matrix, transition_matrix }
    }
}

pub struct HMM {
    params: HMMParams
}

impl HMM {
    pub fn new(params: HMMParams) -> Self {
        HMM { params }
    }

    pub fn kernel(&self, rng: &mut ThreadRng, trace: &mut HMMTrace, state_probs: Vec<f64>, new_observation: usize) {
        let new_state = categorical.random(rng, state_probs.clone());
        let obs_probs = self.params.emission_matrix.column(new_state).transpose().data.as_vec().to_vec();
        trace.extend(new_state, new_observation);
        trace.set_score(trace.get_score() + categorical.logpdf(&new_observation, obs_probs));
    }
}

impl GenerativeFunction for HMM {
    type X = (i64, ParamStore);
    type T = Vec<Rc<usize>>;
    type U = HMMTrace;

    fn simulate(&self, _: &mut ThreadRng, _: Self::X) -> Self::U {
        panic!("not implemented");
    }

    fn generate(&self, rng: &mut ThreadRng, args: Self::X, constraints: impl ChoiceBuffer) -> Self::U {
        let (t, _) = args;
        if t != 1 {
            panic!("only expect generate to be called to initialize the state (T = 1)");
        }
        let new_observation = *(constraints.get_value(Box::leak(format!("{} => observation", 1).into_boxed_str())) as &dyn Any)
            .downcast_ref::<Rc<usize>>()
            .unwrap()
            .clone();
        let mut trace = HMMTrace::new();
        let state_probs = self.params.prior.data.as_vec().to_vec();
        self.kernel(rng, &mut trace, state_probs, new_observation);
        trace
    }

    fn propose(&self, _: &mut ThreadRng, _: Self::X) -> (ChoiceHashMap<Self::T>, f64) {
        panic!("not implemented");
    }

    fn assess(&self, _: &mut ThreadRng, _: Self::X, _: impl ChoiceBuffer) -> f64 {
        panic!("not implemented");
    }

    fn update(&self, rng: &mut ThreadRng, trace: &mut Self::U, _: Self::X, diff: gen_rs::GfDiff, constraints: impl ChoiceBuffer) -> ChoiceHashMap<Self::T> {
        match diff {
            GfDiff::Extend => { 
                let new_observation = *(constraints.get_value(Box::leak(format!("{} => observation", trace.get_t()+1).into_boxed_str())) as &dyn Any)
                    .downcast_ref::<Rc<usize>>()
                    .unwrap()
                    .clone();
                let prev_state = **trace.get_choices().get_value(Box::leak(format!("{} => state", trace.get_t()).into_boxed_str()));
                let state_probs = self.params.transition_matrix.column(prev_state)
                    .transpose()
                    .data
                    .as_vec()
                    .to_vec();
                self.kernel(rng, trace, state_probs, new_observation);
            },
            _ => { panic!("Can't handle GF change type: {:?}", diff) },
        }
        ChoiceHashMap::new()
    }
}