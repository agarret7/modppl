#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(associated_const_equality)]
#![feature(closure_lifetime_binder)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(box_into_inner)]
#![feature(map_try_insert)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod gfi;
pub mod gfi_new;
pub mod trie;
// pub mod hashmap;
pub mod choices;
// pub mod dynamic;
pub mod modeling;
pub mod inference;
pub mod mathutils;
pub mod address;

// modeling interface
pub use trie::Trie;
pub use modeling::dists::{self,Sample};
pub use gfi_new::{StrRec, GenFn};
pub use address::SplitAddr;
// pub use gfi::Addr;
pub use gfi::Trace;
pub use gfi::GenerativeFunction;
pub use gfi::GfDiff;
pub use gfi::ChoiceBuffer;
pub use choices::ChoiceHashMap;

// inference interface
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;
pub use inference::mh;
pub use inference::ParticleSystem;