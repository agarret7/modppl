//! `gen_rs` is a a library for general-purpose, low-level probabilistic modeling and inference.
//! Modeling and inference are separated by a trait interface, called `GenFn`.
//! 
//! Any function that implements `GenFn` (representing a Bayesian model) can use and compose
//! any inference procedure in the standard inference library.

#![deny(missing_docs)]
#![allow(non_upper_case_globals)]

extern crate approx;
extern crate nalgebra;
extern crate rand;
extern crate regex;


use std::cell::RefCell;
use rand::rngs::ThreadRng;


thread_local!(
    /// Forked PRNG, accessible as a static crate-level thread-local constant. (Use like `GLOBAL_RNG.with_borrow_mut(|rng| { ... })`).
    pub static GLOBAL_RNG: RefCell<ThreadRng> = RefCell::new(ThreadRng::default())
);


/// Definition of the Generative Function Interface (GFI).
pub mod gfi;

/// Utilities for parsing addresses (special keys used in the `Trie` data structure).
pub mod address;

/// Implementations of the `Trie` data structure, used extensively in `modeling::triefn`. 
pub mod trie;

/// Distributions and a modeling DSL built on `Trie`s.
pub mod modeling;

/// Standard inference library.
pub mod inference;

mod mathutils;

// modeling libs
pub use trie::Trie;
pub use address::{SplitAddr, normalize_addr};
pub use gfi::{Trace, GenFn, GfDiff};
pub use modeling::dists::{u01,Distribution,bernoulli,uniform_continuous,uniform,uniform_discrete,categorical,normal,mvnormal};
pub use modeling::triefn::{TrieFn,TrieFnState,AddrTrie};
pub use modeling::unfold::Unfold;
pub use mathutils::logsumexp;

// inference libs
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;
pub use inference::mh;
pub use inference::ParticleSystem;