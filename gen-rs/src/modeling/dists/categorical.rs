use rand::rngs::ThreadRng;
use approx;
use super::{Distribution,u01};


pub struct Categorical { }
pub const categorical: Categorical = Categorical { };

impl Distribution<usize,Vec<f64>> for Categorical {
    fn logpdf(&self, x: &usize, probs: Vec<f64>) -> f64 {
        approx::assert_abs_diff_eq!(probs.iter().sum::<f64>(), 1.0, epsilon = 1e-8);
        return if *x < probs.len() {
            probs[*x].ln()
        } else {
            -f64::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, probs: Vec<f64>) -> usize {
        approx::assert_abs_diff_eq!(probs.iter().sum::<f64>(), 1.0, epsilon = 1e-8);
        let u = u01(rng);
        let mut t = 0.;
        let mut x: usize = 0;
        while t < u {
            t += probs[x];
            x += 1;
        }
        return x - 1;
    }
}