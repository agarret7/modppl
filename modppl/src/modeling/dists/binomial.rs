use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Binomial as BinomialSampler, Distribution as _};

/// Binomial distribution type
pub struct Binomial {}

/// Instantiation of the binomial distribution
pub const binomial: Binomial = Binomial {};

impl Distribution<i64, (i64, Real)> for Binomial {
    fn logpdf(&self, k: &i64, params: (i64, Real)) -> Real {
        let (n, p) = params;
        let log_binom = (0..*k)
            .map(|i| ((n - i) as Real).ln() - ((i + 1) as Real).ln())
            .sum::<Real>();
        log_binom + (*k as Real) * p.ln() + ((n - k) as Real) * (1. - p).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (i64, Real)) -> i64 {
        let (n, p) = params;
        let binom_sampler = BinomialSampler::new(n as u64, p as f64).ok().unwrap();
        binom_sampler.sample(rng) as i64
    }
}
