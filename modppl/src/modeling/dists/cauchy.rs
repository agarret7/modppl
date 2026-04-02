use rand::rngs::ThreadRng;
use super::Distribution;
use std::f64::consts::PI;
use rand_distr::{
    Distribution as _,
    Cauchy as CauchySampler
};


/// Cauchy distribution type
pub struct Cauchy { }

/// Instantiation of the cauchy distribution
pub const cauchy: Cauchy = Cauchy { };

impl Distribution<f64,(f64,f64)> for Cauchy {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (x0, gamma) = params;
        -PI.ln() - gamma.ln() - (1. + ((x - x0)/gamma).powi(2)).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (x0, gamma) = params;
        let cauchy_sampler = CauchySampler::new(x0, gamma).ok().unwrap();
        cauchy_sampler.sample(rng)
    }
}
