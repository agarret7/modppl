use std::fmt::Display;
use rand::rngs::ThreadRng;
use super::{Distribution,u01};


fn check_bounds<T: PartialOrd + Display>(a: T, b: T) {
    if a >= b {
        panic!("a >= b in [a, b] = [{}, {}]; b > a is required.", a, b);
    }
}

/// Uniform continuous distribution type
pub struct UniformContinuous { }

/// Instantiation of the uniform continuous distribution
pub const uniform_continuous: UniformContinuous = UniformContinuous { };

/// Alias for uniform_continuous
pub const uniform: UniformContinuous = uniform_continuous;

impl Distribution<f64,(f64,f64)> for UniformContinuous {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        check_bounds(a, b);
        if a <= *x && *x <= b { -(b - a).ln() } else { f64::NEG_INFINITY }
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (a, b) = params;
        check_bounds(a, b);
        u01(rng) * (b - a) + a
    }
}


/// Uniform discrete distribution type
pub struct UniformDiscrete { }

/// Instantiation of the uniform discrete distribution
pub const uniform_discrete: UniformDiscrete = UniformDiscrete { };

impl Distribution<i64,(i64,i64)> for UniformDiscrete {
    fn logpdf(&self, x: &i64, params: (i64,i64)) -> f64 {
        let (a, b) = params;
        check_bounds(a, b);
        if a <= *x && *x <= b { -((b - a + 1) as f64).ln() } else { f64::NEG_INFINITY }
    }

    fn random(&self, rng: &mut ThreadRng, params: (i64,i64)) -> i64 {
        let (a, b) = params;
        check_bounds(a, b);
        (u01(rng) * (b - a + 1) as f64).trunc() as i64 + a
    }
}