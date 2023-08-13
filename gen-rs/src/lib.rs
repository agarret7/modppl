// #![feature(return_position_impl_trait_in_trait)]
// #![feature(associated_type_defaults)]
// #![feature(associated_const_equality)]
// #![feature(closure_lifetime_binder)]
// #![feature(anonymous_lifetime_in_impl_trait)]
// #![feature(box_into_inner)]
// #![feature(map_try_insert)]
// #![feature(unboxed_closures)]
// #![feature(fn_traits)]
#![feature(local_key_cell_methods)]

use std::cell::RefCell;
use rand::rngs::ThreadRng;

thread_local!(pub static GLOBAL_RNG: RefCell<ThreadRng> = RefCell::new(ThreadRng::default()));


pub mod gfi;
pub mod trie;
pub mod modeling;
pub mod inference;
pub mod mathutils;
pub mod address;

// modeling libs
pub use trie::Trie;
pub use modeling::dists::{self,Sample};
pub use gfi::{Trace, GenFn, GfDiff};
pub use address::{SplitAddr, normalize_addr};

// inference libs
pub use inference::importance_sampling;
// pub use inference::metropolis_hastings;
// pub use inference::mh;
// pub use inference::ParticleSystem;