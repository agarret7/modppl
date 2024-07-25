use rand::rngs::ThreadRng;
use approx;
use super::{Distribution,u01};


/// Categorical distribution type
pub struct Categorical { }

/// Instantiation of the categorical distribution
pub const categorical: Categorical = Categorical { };

impl Distribution<i64,Vec<f64>> for Categorical {
    fn logpdf(&self, x: &i64, probs: Vec<f64>) -> f64 {
        approx::assert_abs_diff_eq!(probs.iter().sum::<f64>(), 1.0, epsilon = 1e-8);
        return if *x < probs.len() as i64 {
            probs[*x as usize].ln()
        } else {
            f64::NEG_INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, probs: Vec<f64>) -> i64 {
        approx::assert_abs_diff_eq!(probs.iter().sum::<f64>(), 1.0, epsilon = 1e-8);
        let u = u01(rng);
        let mut t = 0.;
        let mut x: i64 = 0;
        while t < u {
            t += probs[x as usize];
            x += 1;
        }
        return x - 1;
    }
}