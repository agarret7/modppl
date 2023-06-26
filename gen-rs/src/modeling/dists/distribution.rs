use rand::{self,Rng, rngs::ThreadRng};


pub fn u01(rng: &mut ThreadRng) -> f64 {
    rng.sample(rand::distributions::Uniform::new(0., 1.))
}

pub trait Distribution<T,U> {
    fn logpdf(&self, x: &T, params: &U) -> f64;
    fn random(&self, rng: &mut ThreadRng, params: &U) -> T;
}