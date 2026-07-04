use super::log_gamma_fn;
use super::Distribution;
use crate::Real;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution as _, Gamma as GammaSampler};

/// Dirichlet distribution type
pub struct Dirichlet {}

/// Instantiation of the dirichlet distribution
pub const dirichlet: Dirichlet = Dirichlet {};

impl Distribution<Vec<Real>, Vec<Real>> for Dirichlet {
    fn logpdf(&self, x: &Vec<Real>, alphas: Vec<Real>) -> Real {
        let sum_alpha: Real = alphas.iter().sum();
        let log_beta =
            alphas.iter().map(|&a| log_gamma_fn(a)).sum::<Real>() - log_gamma_fn(sum_alpha);
        let log_numerator: Real = alphas
            .iter()
            .zip(x.iter())
            .map(|(&a, &xi)| (a - 1.) * xi.ln())
            .sum();
        log_numerator - log_beta
    }

    fn random(&self, rng: &mut ThreadRng, alphas: Vec<Real>) -> Vec<Real> {
        let samples: Vec<Real> = alphas
            .iter()
            .map(|&a| {
                let g = GammaSampler::new(a, 1.).ok().unwrap();
                g.sample(rng)
            })
            .collect();
        let sum: Real = samples.iter().sum();
        samples.iter().map(|&s| s / sum).collect()
    }
}
