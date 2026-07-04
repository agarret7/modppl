use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Geometric as GeometricSampler};

/// Geometric distribution type
pub struct Geometric {}

/// Instantiation of the geometric distribution
pub const geometric: Geometric = Geometric {};

impl Distribution<i64, Real> for Geometric {
    fn logpdf(&self, k: &i64, p: Real) -> Real {
        debug_assert!(0. < p && p < 1.);
        ((1. - p).powf(*k as Real) * p).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: Real) -> i64 {
        debug_assert!(0. < p && p < 1.);
        let geometric_sampler = GeometricSampler::new(p as f64).ok().unwrap();
        geometric_sampler.sample(rng) as i64
    }
}
