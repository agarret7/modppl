mod distribution;

mod bernoulli;
mod categorical;
mod normal;
mod mvnormal;


pub use self::distribution::{u01,Distribution,Sample};
pub use {self::bernoulli::*,self::categorical::*,self::normal::*,self::mvnormal::*};