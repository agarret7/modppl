use rand::rngs::ThreadRng;
use super::{Distribution,u01};


pub struct Categorical { }
pub const categorical: Categorical = Categorical { };

impl Distribution<usize,Vec<f64>> for Categorical {
    fn logpdf(&self, x: &usize, probs: Vec<f64>) -> f64 {
        return if (*x as i32) > 0 && (*x as i32) <= probs.len() as i32 {
            probs[*x].ln()
        } else {
            -f64::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, probs: Vec<f64>) -> usize {
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