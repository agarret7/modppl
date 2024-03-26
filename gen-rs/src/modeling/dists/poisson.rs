use rand::rngs::ThreadRng;
use super::Distribution;
use rand_distr::{
    Distribution as _,
    Poisson as PoissonSampler
};


fn factorial(i: i64) -> i64 {
    if i <= 1 { 1 } else { i * factorial(i-1) }
}

/// Poisson distribution type
pub struct Poisson { }

/// Instantiation of the poisson distribution
pub const poisson: Poisson = Poisson { };

impl Distribution<i64,f64> for Poisson {
    fn logpdf(&self, k: &i64, rate: f64) -> f64 {
        (*k as f64)*rate.ln() - rate - factorial(*k) as f64
    }

    fn random(&self, rng: &mut ThreadRng, rate: f64) -> i64 {
        let poisson_sampler = PoissonSampler::new(rate).ok().unwrap();
        poisson_sampler.sample(rng) as i64
    }
}