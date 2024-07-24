use nalgebra::{DVector,DMatrix};
use rand::rngs::ThreadRng;

use super::{HMMTrace,ParamStore,extend};
use gen_rs::{GenFn,GfDiff,Distribution,categorical};


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

    pub fn kernel(&self, trace: &mut HMMTrace, state_probs: Vec<f64>, new_observation: usize) -> f64 {
        let mut rng = ThreadRng::default();
        let new_state = categorical.random(&mut rng, state_probs.clone()) as usize;
        let obs_probs = self.params.emission_matrix.column(new_state).transpose().data.as_vec().to_vec();
        extend(trace, new_state, new_observation);
        let weight = categorical.logpdf(&(new_observation as i64), obs_probs);
        trace.logjp += weight;
        weight
    }
}

impl GenFn<(i64,ParamStore),(Vec<Option<usize>>,Vec<Option<usize>>),Vec<usize>> for HMM {

    fn simulate(&self, _: (i64, ParamStore)) -> HMMTrace {
        panic!("not implemented");
    }

    fn generate(&self, args: (i64, ParamStore), constraints: (Vec<Option<usize>>,Vec<Option<usize>>)) -> (HMMTrace, f64) {
        let (t, _) = args;
        if t != 1 {
            panic!("only expect generate to be called to initialize the state (T = 1)");
        }
        let new_observation = constraints.1[0].unwrap();
        let mut trace = HMMTrace::new(args, constraints, vec![new_observation], 0.);
        let state_probs = self.params.prior.data.as_vec().to_vec();
        let weight = self.kernel(&mut trace, state_probs, new_observation);
        (trace, weight)
    }

    fn update(&self, mut trace: HMMTrace, _: (i64, ParamStore), diff: gen_rs::GfDiff, constraints: (Vec<Option<usize>>,Vec<Option<usize>>))
        -> (HMMTrace, (Vec<Option<usize>>, Vec<Option<usize>>), f64)
    {
        match diff {
            GfDiff::Extend => {
                let new_observation = constraints.1.last().unwrap().unwrap();
                let prev_state = trace.data.0.last().unwrap().unwrap();
                let state_probs = self.params.transition_matrix.column(prev_state)
                    .transpose()
                    .data
                    .as_vec()
                    .to_vec();
                let weight = self.kernel(&mut trace, state_probs, new_observation);
                (trace, (vec![], vec![]), weight)
            },
            _ => { panic!("Can't handle GF change type: {:?}", diff) },
        }
    }

}