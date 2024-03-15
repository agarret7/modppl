use rand::rngs::ThreadRng;
use super::Distribution;
use rand_distr::{
    Distribution as _,
    Poisson as PoissonSampler
};


/// Poisson distribution type
pub struct Poisson { }

/// Instantiation of the poisson distribution
pub const poisson: Poisson = Poisson { };

impl Distribution<f64,f64> for Poisson {
    fn logpdf(&self, x: &f64, rate: f64) -> f64 {
        panic!("not implemented");
    }

    fn random(&self, rng: &mut ThreadRng, rate: f64) -> f64 {
        let gamma_sampler = PoissonSampler::new(rate).ok().unwrap();
        gamma_sampler.sample(rng)
    }
}