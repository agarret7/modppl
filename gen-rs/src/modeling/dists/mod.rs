mod distribution;

mod bernoulli;
mod uniform;
mod categorical;
mod normal;
mod mvnormal;


pub use self::distribution::{u01,Distribution,Sample};
pub use {self::bernoulli::*,self::uniform::*,self::categorical::*,self::normal::*,self::mvnormal::*};