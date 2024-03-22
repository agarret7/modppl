use rand::{Rng,rngs::ThreadRng};
use crate::{Trace, GenFn, GfDiff, DynTrie};


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

use crate::{DynGenFn,DynGenFnHandler};

fn _dist_genfn<'a,T: 'static + Clone,U: 'static + Clone>(g: &'a mut DynGenFnHandler<(impl Distribution<T,U> + 'static,U,String),T>, args: (impl Distribution<T,U>,U,String)) -> T {
    let (dist, params, addr) = args;
    g.sample_at(&dist, params, &addr)
}
pub const fn dist_genfn<T: 'static + Clone,U: 'static + Clone,D: Distribution<T,U> + 'static>() -> DynGenFn<(D,U,String),T> {
    DynGenFn { func: _dist_genfn }
}


// use std::rc::Rc;

// impl<U: Clone,T: 'static + Clone,D: Distribution<T,U>> GenFn<U,DynTrie,T> for D {
//     fn simulate(&self, args: U) -> Trace<U,DynTrie,T> {
//         let mut prng = ThreadRng::default();
//         let x = self.random(&mut prng, args.clone());
//         let logp = self.logpdf(&x, args.clone());
//         let retv = Some(x.clone());
//         Trace { args: args, data: DynTrie::leaf(Rc::new(x), logp), retv, logp }
//     }

//     fn generate(&self, args: U, constraints: DynTrie) -> (Trace<U,DynTrie,T>, f64) {
//         let x: Rc<T>;
//         if constraints.is_empty() {
//             let mut prng = ThreadRng::default();
//             x = Rc::new(self.random(&mut prng, args.clone()));
//         } else {
//             x = constraints.unwrap_inner_unchecked().downcast::<T>().ok().unwrap();
//         }
//         let logp = self.logpdf(&x, args.clone());
//         let retv = Some(x.as_ref().clone());
//         (Trace { args: args, data: DynTrie::leaf(x, logp), retv, logp }, logp)
//     }

//     fn update(&self,
//         _: Trace<U,DynTrie,T>,
//         _: U,
//         _: GfDiff,
//         _: DynTrie
//     ) -> (Trace<U,DynTrie,T>, DynTrie, f64) {
//         panic!("not implemented")
//     }
// }