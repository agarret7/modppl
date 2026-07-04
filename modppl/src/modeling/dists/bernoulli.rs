use super::{u01, Distribution};
use crate::Real;
use rand::rngs::ThreadRng;

/// Bernoulli distribution type
pub struct Bernoulli {}

/// Instantiation of the Bernoulli distribution
pub const bernoulli: Bernoulli = Bernoulli {};

impl Distribution<bool, Real> for Bernoulli {
    fn logpdf(&self, a: &bool, p: Real) -> Real {
        (if *a { p } else { 1. - p }).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: Real) -> bool {
        p > u01(rng)
    }
}
