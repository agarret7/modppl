use super::log_gamma_fn;
use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Beta as BetaSampler, Distribution as _};

/// Beta distribution type
pub struct Beta {}

/// Instantiation of the beta distribution
pub const beta: Beta = Beta {};

impl Distribution<Real, (Real, Real)> for Beta {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (a, b) = params;
        log_gamma_fn(a + b) - log_gamma_fn(a) - log_gamma_fn(b)
            + (a - 1.) * x.ln()
            + (b - 1.) * (1. - x).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (a, b) = params;
        let beta_sampler = BetaSampler::new(a, b).ok().unwrap();
        beta_sampler.sample(rng)
    }
}
