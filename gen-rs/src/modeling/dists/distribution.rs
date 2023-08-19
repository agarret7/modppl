use rand::{self,Rng,rngs::ThreadRng};
use crate::{GLOBAL_RNG, Trace, GenFn, GfDiff};


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

/// Represents a (traced) sample of type `T` from a distribution.
pub struct Sample<T>(pub T);

impl<U: Clone,T: Clone,D: Distribution<T,U>> GenFn<U,Sample<T>,T> for D {
    fn simulate(&self, args: U) -> Trace<U,Sample<T>,T> {
        let x = GLOBAL_RNG.with_borrow_mut(|rng| {
            self.random(rng, args.clone())
        });
        let logp = self.logpdf(&x, args.clone());
        Trace { args: args, data: Sample(x.clone()), retv: Some(x), logp }
    }

    fn generate(&self, args: U, constraints: Sample<T>) -> (Trace<U,Sample<T>,T>, f64) {
        let x = constraints.0;
        let logp = self.logpdf(&x, args.clone());
        (Trace { args: args, data: Sample(x.clone()), retv: Some(x), logp }, logp)
    }

    fn update(&self,
        _: Trace<U,Sample<T>,T>,
        _: U,
        _: GfDiff,
        _: Sample<T>
    ) -> (Trace<U,Sample<T>,T>, Sample<T>, f64) {
        panic!("not implemented")
    }
}