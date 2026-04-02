use rand::rngs::ThreadRng;
use super::{Distribution, u01};


/// Laplace distribution type
pub struct Laplace { }

/// Instantiation of the laplace distribution
pub const laplace: Laplace = Laplace { };

impl Distribution<f64,(f64,f64)> for Laplace {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (mu, b) = params;
        -(2_f64).ln() - b.ln() - (x - mu).abs()/b
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (mu, b) = params;
        let u = u01(rng) - 0.5;
        mu - b * u.signum() * (1. - 2.*u.abs()).ln()
    }
}
