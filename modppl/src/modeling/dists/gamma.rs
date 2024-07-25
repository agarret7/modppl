use rand::rngs::ThreadRng;
use super::Distribution;
use compute::functions::gamma as gamma_f;
use rand_distr::{
    Distribution as _,
    Gamma as GammaSampler
};


/// Gamma distribution type
pub struct Gamma { }

/// Instantiation of the gamma distribution
pub const gamma: Gamma = Gamma { };

impl Distribution<f64,(f64,f64)> for Gamma {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        (a-1.)*x.ln() - x/b - gamma_f(a).ln() - a*b.ln() 
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        let gamma_sampler = GammaSampler::new(a, b).ok().unwrap();
        gamma_sampler.sample(rng)
    }
}