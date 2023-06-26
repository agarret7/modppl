use rand::rngs::ThreadRng;
use std::rc::Rc;
use ndarray::{Array1,Array2};
use gen_rs::{
    GenerativeFunction, ChoiceBuffer, ChoiceHashMap
};


pub use super::trace::ParticleFilterState;

pub struct ParticleFilterModel {
    transition: Array2<f64>,
    emission: Array2<f64>,

    process_cov: Array2<f64>,
    obs_cov: Array2<f64>,
}

struct ParticleFilterArgs {
    T: u32,
    init_latent: Array1<f64>,
}

impl GenerativeFunction for ParticleFilterModel {
    type X = ParticleFilterArgs;
    type T = (Vec<Rc<Array1<f64>>>, Vec<Rc<Array1<f64>>>);
    type U = ParticleFilterState;

    fn simulate(&self, rng: &mut ThreadRng, args: Rc<ParticleFilterArgs>) -> Self::U {
        let mut latents = Vec::new();
        let mut observations = Vec::new();
        let prev_latent = args.init_latent;
        latents.push(prev_latent.clone());
        for t in 0..args.T {
            let new_latent = self.transition.dot(&prev_latent);
            latents.push(new_latent);
            let observation = self.emission.dot(&prev_latent);
            observations.push(observation);
            prev_latent = new_latent;
        }
        ParticleFilterState {
            self.transition,
            self.emission,
            self.process_cov,
            self.obs_cov
        }
    }

    fn generate(&self, rng: &mut ThreadRng, T: Rc<u32>, constraints: impl ChoiceBuffer) -> Self::U {
    }

    fn propose(&self, _: &mut ThreadRng, _: Rc<Self::X>) -> (ChoiceHashMap<Array1<f64>>, f64) {
        // this is wrong, but we don't call propose on this GF.
        (ChoiceHashMap::new(), 0.)
    }

    fn assess(&self, _: &mut ThreadRng, _: Rc<Self::X>, _: impl ChoiceBuffer) -> f64 {
        // this is wrong, but we don't call assess on this GF.
        return 0.
    }

    fn update(&self, trace: Rc<Self::U>, constraints: impl ChoiceBuffer) -> (Self::U, ChoiceHashMap<Array1<f64>>) {
    }
}