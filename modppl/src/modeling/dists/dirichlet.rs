use rand::rngs::ThreadRng;
use super::Distribution;
use compute::functions::gamma as gamma_f;
use rand_distr::{
    Distribution as _,
    Gamma as GammaSampler
};


/// Dirichlet distribution type
pub struct Dirichlet { }

/// Instantiation of the dirichlet distribution
pub const dirichlet: Dirichlet = Dirichlet { };

impl Distribution<Vec<f64>,Vec<f64>> for Dirichlet {
    fn logpdf(&self, x: &Vec<f64>, alphas: Vec<f64>) -> f64 {
        let sum_alpha: f64 = alphas.iter().sum();
        let log_beta = alphas.iter().map(|&a| gamma_f(a).ln()).sum::<f64>() - gamma_f(sum_alpha).ln();
        let log_numerator: f64 = alphas.iter().zip(x.iter()).map(|(&a, &xi)| (a - 1.)*xi.ln()).sum();
        log_numerator - log_beta
    }

    fn random(&self, rng: &mut ThreadRng, alphas: Vec<f64>) -> Vec<f64> {
        let samples: Vec<f64> = alphas.iter().map(|&a| {
            let g = GammaSampler::new(a, 1.).ok().unwrap();
            g.sample(rng)
        }).collect();
        let sum: f64 = samples.iter().sum();
        samples.iter().map(|&s| s/sum).collect()
    }
}
