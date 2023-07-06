mod distribution;
mod bernoulli;
mod categorical;
mod normal;
mod mvnormal;

pub use distribution::{u01,Distribution};
pub use bernoulli::{Bernoulli,bernoulli};
pub use categorical::{Categorical,categorical};
pub use normal::{Normal,normal};
pub use mvnormal::{MvNormal,mvnormal};