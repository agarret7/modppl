use super::{u01, Distribution};
use crate::Real;
use rand::rngs::ThreadRng;

/// Laplace distribution type
pub struct Laplace {}

/// Instantiation of the laplace distribution
pub const laplace: Laplace = Laplace {};

impl Distribution<Real, (Real, Real)> for Laplace {
    fn logpdf(&self, x: &Real, params: (Real, Real)) -> Real {
        let (mu, b) = params;
        -(2.0 as Real).ln() - b.ln() - (x - mu).abs() / b
    }

    fn random(&self, rng: &mut ThreadRng, params: (Real, Real)) -> Real {
        let (mu, b) = params;
        let u = u01(rng) - 0.5;
        mu - b * u.signum() * (1. - 2. * u.abs()).ln()
    }
}
