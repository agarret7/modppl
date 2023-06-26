use rand::rngs::ThreadRng;
use super::{Distribution,u01};


pub struct Categorical { }
pub const categorical: Categorical = Categorical { };

impl Distribution<usize,Vec<f32>> for Categorical {
    fn logpdf(&self, x: &usize, probs: &Vec<f32>) -> f32 {
        return if (*x as i32) > 0 && (*x as i32) <= probs.len() as i32 {
            probs[*x].ln()
        } else {
            -f32::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, probs: &Vec<f32>) -> usize {
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