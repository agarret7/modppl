use rand::rngs::ThreadRng;
use super::Distribution;
use compute::functions::gamma as gamma_f;
use rand_distr::{
    Distribution as _,
    Gamma as GammaSampler
};


/// Inverse Gamma distribution type
pub struct InvGamma { }

/// Instantiation of the inverse gamma distribution
pub const inv_gamma: InvGamma = InvGamma { };

impl Distribution<f64,(f64,f64)> for InvGamma {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        a*b.ln() - gamma_f(a).ln() - (a+1.)*x.ln() - b/x
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        let gamma_sampler = GammaSampler::new(a, 1./b).ok().unwrap();
        1. / gamma_sampler.sample(rng)
    }
}
