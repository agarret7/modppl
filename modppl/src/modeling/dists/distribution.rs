use rand::{Rng,rngs::ThreadRng};


/// Sample a random variable uniformly in the interval [0., 1.].
pub fn u01(rng: &mut ThreadRng) -> f64 {
    rng.sample(rand::distributions::Uniform::new(0., 1.))
}

/// Trait for sampling distributions with an analytically calculable probability density function (pdf).
pub trait Distribution<T,U> {

    /// Return the `log[p(x; params)]`.
    fn logpdf(&self, x: &T, params: U) -> f64;

    /// Sample a random value `x ~ p(. ; params)`.
    fn random(&self, rng: &mut ThreadRng, params: U) -> T;

}