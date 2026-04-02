use rand::rngs::ThreadRng;
use super::Distribution;
use rand_distr::{
    Distribution as _,
    Exp as ExpSampler
};


/// Exponential distribution type
pub struct Exponential { }

/// Instantiation of the exponential distribution
pub const exponential: Exponential = Exponential { };

impl Distribution<f64,f64> for Exponential {
    fn logpdf(&self, x: &f64, rate: f64) -> f64 {
        rate.ln() - rate*x
    }

    fn random(&self, rng: &mut ThreadRng, rate: f64) -> f64 {
        let exp_sampler = ExpSampler::new(rate).ok().unwrap();
        exp_sampler.sample(rng)
    }
}
