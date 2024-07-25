//! `gen_rs` is a a library for general-purpose, low-level probabilistic modeling and inference.
//! Modeling and inference are separated by a trait interface, called `GenFn`.
//! 
//! Any function that implements `GenFn` (representing a Bayesian model) can use and compose
//! any inference procedure in the standard inference library.

// #![deny(missing_docs)]
#![allow(non_upper_case_globals)]

extern crate approx;
extern crate nalgebra;
extern crate rand;
extern crate regex;

pub mod prelude;

/// Definition of the Generative Function Interface (GFI).
pub mod gfi;

/// Utilities for parsing addresses (keys used in the `Trie` data structure).
pub mod address;

/// Implementations of the `Trie` data structure, used extensively in `modeling::DynGenFn`. 
pub mod trie;

/// Distributions and a modeling DSL built on `Trie`s.
pub mod modeling;

/// Standard inference library.
pub mod inference;

/// For an input vector of `[x1, ..., xn]`, return `log(exp(x1) + ... + exp(xn))`.
pub fn logsumexp(xs: &Vec<f64>) -> f64 {
    let max = xs.iter().cloned().fold(-1./0. /* -inf */, f64::max);
    if max == f64::NEG_INFINITY {
        f64::NEG_INFINITY
    } else {
        let mut sum_exp = 0.;
        for x in xs {
            sum_exp += (x - max).exp();
        }
        max + sum_exp.ln()
    }
}

// modeling libs
pub use trie::Trie;
pub use address::{SplitAddr, AddrMap, normalize_addr};
pub use gfi::{Trace, GenFn,ArgDiff};
pub use modeling::dists::{
    u01,Distribution,
    bernoulli,
    uniform_continuous,
    uniform,
    uniform_discrete,
    categorical,
    normal,
    mvnormal,
    poisson,
    gamma,
    beta
};
pub use modeling::dyngenfn::{DynTrie,DynTrace,DynGenFn,DynGenFnHandler};
pub use modeling::dynunfold::{DynUnfold,DynParticles};

// inference libs
pub use inference::{importance_sampling, importance_resampling};
pub use inference::{metropolis_hastings, mh, regenerative_metropolis_hastings, regen_mh};
pub use inference::ParticleSystem;