use super::log_gamma_fn;
use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Gamma as GammaSampler};

/// Inverse Gamma distribution type
pub struct InvGamma {}

/// Instantiation of the inverse gamma distribution
pub const inv_gamma: InvGamma = InvGamma {};

impl Distribution<Real, (Real, Real)> for InvGamma {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (a, b) = params;
        a * b.ln() - log_gamma_fn(a) - (a + 1.) * x.ln() - b / x
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (a, b) = params;
        let gamma_sampler = GammaSampler::new(a, 1. / b).ok().unwrap();
        1. / gamma_sampler.sample(rng)
    }
}
