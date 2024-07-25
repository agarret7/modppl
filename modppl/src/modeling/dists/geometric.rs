use rand::rngs::ThreadRng;
use super::Distribution;
use rand_distr::{
    Distribution as _,
    Geometric as GeometricSampler
};


/// Geometric distribution type
pub struct Geometric { }

/// Instantiation of the geometric distribution
pub const geometric: Geometric = Geometric { };

impl Distribution<i64,f64> for Geometric {
    fn logpdf(&self, k: &i64, p: f64) -> f64 {
        debug_assert!(0. < p && p < 1.);
        ((1. - p).powf(*k as f64)*p).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: f64) -> i64 {
        debug_assert!(0. < p && p < 1.);
        let geometric_sampler = GeometricSampler::new(p).ok().unwrap();
        geometric_sampler.sample(rng) as i64
    }
}