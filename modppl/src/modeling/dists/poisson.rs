use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Poisson as PoissonSampler};

/// Poisson distribution type
pub struct Poisson {}

/// Instantiation of the poisson distribution
pub const poisson: Poisson = Poisson {};

impl Distribution<i64, Real> for Poisson {
    fn logpdf(&self, k: &i64, rate: Real) -> Real {
        (*k as Real) * rate.ln() - rate - (1..=*k).map(|v| (v as Real).ln()).sum::<Real>()
    }

    fn random(&self, rng: &mut ThreadRng, rate: Real) -> i64 {
        let poisson_sampler = PoissonSampler::new(rate).ok().unwrap();
        poisson_sampler.sample(rng) as i64
    }
}
