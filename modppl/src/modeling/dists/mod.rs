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
    self::gamma::*
};