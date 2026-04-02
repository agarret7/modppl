mod distribution;

mod bernoulli;
mod uniform;
mod categorical;
mod normal;
mod mvnormal;
mod poisson;
mod geometric;
mod gamma;
mod beta;
mod inv_gamma;
mod binomial;
mod exponential;
mod laplace;
mod cauchy;
mod dirichlet;


pub use self::distribution::{u01,Distribution};
pub use {
    self::bernoulli::*,
    self::uniform::*,
    self::categorical::*,
    self::normal::*,
    self::mvnormal::*,
    self::geometric::*,
    self::poisson::*,
    self::beta::*,
    self::gamma::*,
    self::inv_gamma::*,
    self::binomial::*,
    self::exponential::*,
    self::laplace::*,
    self::cauchy::*,
    self::dirichlet::*
};