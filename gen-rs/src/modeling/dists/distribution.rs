use rand::{self,Rng, rngs::ThreadRng};
use super::Sample;
use crate::{GenFn, gfi_new::TraceNew as Trace};


pub fn u01(rng: &mut ThreadRng) -> f64 {
    rng.sample(rand::distributions::Uniform::new(0., 1.))
}

pub trait Distribution<T,U> {
    fn logpdf(&self, x: &T, params: U) -> f64;
    fn random(&self, rng: &mut ThreadRng, params: U) -> T;
}

impl<U: Clone,T: Clone,D: Distribution<T,U>> GenFn<U,Sample<T>,T> for D {
    fn rng(&self) -> ThreadRng {
        ThreadRng::default()
    }

    fn simulate(&mut self, args: U) -> Trace<U,Sample<T>,T> {
        let x = self.random(&mut self.rng(), args.clone());
        let logp = self.logpdf(&x, args.clone());
        Trace { args: args, data: Sample(x.clone()), retv: Some(x), logp }
    }

    fn generate(&mut self, args: U, constraints: Sample<T>) -> (Trace<U,Sample<T>,T>, f64) {
        let x = constraints.0;
        let logp = self.logpdf(&x, args.clone());
        (Trace { args: args, data: Sample(x.clone()), retv: Some(x), logp }, logp)
    }

    fn update(&mut self,
            trace: &mut Trace<U,Sample<T>,T>,
            args: U,
            diff: crate::GfDiff,
            constraints: Sample<T>
        ) -> (Sample<T>, f64) {
        panic!("not implemented")
    }
}