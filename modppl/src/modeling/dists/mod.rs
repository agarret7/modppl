use crate::Real;

/// Natural logarithm of the gamma function, evaluated before casting to
/// avoid overflow in `f32` for moderately large inputs.
pub(crate) fn log_gamma_fn(x: Real) -> Real {
    compute::functions::gamma(x as f64).ln() as Real
}

mod distribution;

mod bernoulli;
mod beta;
mod binomial;
mod categorical;
mod cauchy;
mod dirichlet;
mod exponential;
mod gamma;
mod geometric;
mod inv_gamma;
mod laplace;
mod mvnormal;
mod normal;
mod poisson;
mod uniform;

pub use self::distribution::{u01, Distribution};
pub use {
    self::bernoulli::*, self::beta::*, self::binomial::*, self::categorical::*, self::cauchy::*,
    self::dirichlet::*, self::exponential::*, self::gamma::*, self::geometric::*,
    self::inv_gamma::*, self::laplace::*, self::mvnormal::*, self::normal::*, self::poisson::*,
    self::uniform::*,
};
