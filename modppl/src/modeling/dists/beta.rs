use rand::rngs::ThreadRng;
use super::Distribution;
use compute::functions::gamma;
use rand_distr::{
    Distribution as _,
    Beta as BetaSampler
};


/// Beta distribution type
pub struct Beta { }

/// Instantiation of the beta distribution
pub const beta: Beta = Beta { };

impl Distribution<f64,(f64,f64)> for Beta {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (a,b) = params;
        let beta_f = gamma(a + b)/(gamma(a)*gamma(b));
        (beta_f * x.powf(a-1.)*(1.-x).powf(b-1.)).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        let beta_sampler = BetaSampler::new(a, b).ok().unwrap();
        beta_sampler.sample(rng)
    }
}