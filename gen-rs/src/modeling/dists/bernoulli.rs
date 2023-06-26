use rand::rngs::ThreadRng;
use super::{Distribution,u01};


pub struct Bernoulli { }
pub const bernoulli: Bernoulli = Bernoulli { };

impl Distribution<bool,f64> for Bernoulli {
    fn logpdf(&self, a: &bool, p: &f64) -> f64 {
        (if *a { *p } else { 1. - *p }).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: &f64) -> bool {
        *p > u01(rng)
    }
}