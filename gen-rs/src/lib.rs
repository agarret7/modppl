#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(associated_const_equality)]
#![feature(closure_lifetime_binder)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(box_into_inner)]

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
pub use gfi::GfDiff;
pub use choices::ChoiceHashMap;

// inference interface
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;
pub use inference::mh;
pub use inference::ParticleSystem;