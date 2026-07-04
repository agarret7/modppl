use super::{u01, Distribution};
use crate::Real;
use approx;
use rand::rngs::ThreadRng;

/// Categorical distribution type
pub struct Categorical {}

/// Instantiation of the categorical distribution
pub const categorical: Categorical = Categorical {};

impl Distribution<i64, Vec<Real>> for Categorical {
    fn logpdf(&self, x: &i64, probs: Vec<Real>) -> Real {
        approx::assert_abs_diff_eq!(
            probs.iter().sum::<Real>(),
            1.0,
            epsilon = Real::EPSILON.sqrt()
        );
        return if *x < probs.len() as i64 {
            probs[*x as usize].ln()
        } else {
            Real::NEG_INFINITY
        };
    }

    fn random(&self, rng: &mut ThreadRng, probs: Vec<Real>) -> i64 {
        approx::assert_abs_diff_eq!(
            probs.iter().sum::<Real>(),
            1.0,
            epsilon = Real::EPSILON.sqrt()
        );
        let u = u01(rng);
        let mut t = 0.;
        for (x, p) in probs.iter().enumerate() {
            t += *p;
            if u <= t {
                return x as i64;
            }
        }
        (probs.len() - 1) as i64
    }
}
