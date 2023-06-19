#![feature(return_position_impl_trait_in_trait)]
#![feature(impl_trait_in_assoc_type)]

pub mod types_2d;
pub mod gfi;
pub mod choices;
pub mod modeling;
pub mod inference;
pub mod mathutils;

// modeling interface
pub use gfi::Addr;
pub use gfi::ChoiceVal;
pub use gfi::ChoiceBuffer;
pub use gfi::Trace;
pub use gfi::GenerativeFunction;
pub use choices::ChoiceHashMap;

// inference interface
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;