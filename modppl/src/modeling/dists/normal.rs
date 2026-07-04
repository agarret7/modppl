use super::{u01, Distribution};
use crate::real::consts::PI;
use crate::Real;
use rand::rngs::ThreadRng;

/// Gaussian distribution type
pub struct Normal {}

/// Instantiation of the Gaussian distribution
pub const normal: Normal = Normal {};

impl Distribution<Real, (Real, Real)> for Normal {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (mu, std) = params;
        let z = (x - mu) / std;
        -(z.abs().powf(2.) + (2. * PI).ln()) / 2. - std.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (mu, std) = params;
        let u: Real = u01(rng) * 2. - 1.;
        let v: Real = u01(rng) * 2. - 1.;
        let r: Real = u * u + v * v;
        if r == 0. || r > 1. {
            return self.random(rng, params);
        }
        let c = (-2. * r.ln() / r).sqrt();
        return u * c * std + mu;
    }
}
