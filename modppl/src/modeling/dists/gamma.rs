use super::log_gamma_fn;
use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Gamma as GammaSampler};

/// Gamma distribution type
pub struct Gamma {}

/// Instantiation of the gamma distribution
pub const gamma: Gamma = Gamma {};

impl Distribution<Real, (Real, Real)> for Gamma {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (a, b) = params;
        (a - 1.) * x.ln() - x / b - log_gamma_fn(a) - a * b.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (a, b) = params;
        let gamma_sampler = GammaSampler::new(a, b).ok().unwrap();
        gamma_sampler.sample(rng)
    }
}
