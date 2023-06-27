#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(anonymous_lifetime_in_impl_trait)]

pub mod gfi;
pub mod choices;
pub mod modeling;
pub mod inference;
pub mod mathutils;

// modeling interface
pub use modeling::dists;
pub use gfi::Addr;
pub use gfi::ChoiceBuffer;
pub use gfi::Trace;
pub use gfi::GenerativeFunction;
pub use choices::ChoiceHashMap;

// inference interface
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;
pub use inference::mh;