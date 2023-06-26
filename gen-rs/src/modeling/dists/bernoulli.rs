use rand::rngs::ThreadRng;
use super::{Distribution,u01};


pub struct Bernoulli { }
pub const bernoulli: Bernoulli = Bernoulli { };

impl Distribution<bool,f32> for Bernoulli {
    fn logpdf(&self, a: &bool, p: &f32) -> f32 {
        (if *a { *p } else { 1. - *p }).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: &f32) -> bool {
        *p > u01(rng)
    }
}