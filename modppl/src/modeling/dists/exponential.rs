use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Exp as ExpSampler};

/// Exponential distribution type
pub struct Exponential {}

/// Instantiation of the exponential distribution
pub const exponential: Exponential = Exponential {};

impl Distribution<Real, Real> for Exponential {
    fn logpdf(&self, x: &Real, rate: Real) -> Real {
        rate.ln() - rate * x
    }

    fn random(&self, rng: &mut ThreadRng, rate: Real) -> Real {
        let exp_sampler = ExpSampler::new(rate).ok().unwrap();
        exp_sampler.sample(rng)
    }
}
