use super::Distribution;
use crate::real::consts::PI;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Cauchy as CauchySampler, Distribution as _};

/// Cauchy distribution type
pub struct Cauchy {}

/// Instantiation of the cauchy distribution
pub const cauchy: Cauchy = Cauchy {};

impl Distribution<Real, (Real, Real)> for Cauchy {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (x0, gamma) = params;
        -PI.ln() - gamma.ln() - (1. + ((x - x0) / gamma).powi(2)).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (x0, gamma) = params;
        let cauchy_sampler = CauchySampler::new(x0, gamma).ok().unwrap();
        cauchy_sampler.sample(rng)
    }
}
