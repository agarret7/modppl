use rand::rngs::ThreadRng;
use super::{Distribution,u01};


pub struct Poisson { }
pub const poisson: Poisson = Poisson { };

impl Distribution<u64,f64> for Poisson {
    fn logpdf(&self, k: &u64, L: f64) -> f64 {
        (L.powf(*k as f64))/((1..*k).product::<u64>() as f64)*(-L).exp()
    }

    fn random(&self, rng: &mut ThreadRng, L: f64) -> u64 {
        let n = 0;
        let limit: f64;
        let x: u64;
    }
}