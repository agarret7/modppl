//! `modppl` is a a library for general-purpose, low-level probabilistic modeling and inference.
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

#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!(
    "modppl: features `f32` and `f64` are mutually exclusive; \
     enable `f32` with `default-features = false`."
);

/// Floating-point precision selection.
///
/// All values, parameters, and log-probabilities in `modppl` use the [`Real`]
/// alias, which is `f64` by default. Enabling the `f32` feature (with
/// `default-features = false`) switches the whole crate to 32-bit floats:
///
/// ```toml
/// modppl = { version = "0.3", default-features = false, features = ["f32"] }
/// ```
pub mod real {
    /// The floating-point type used for all probabilistic computation.
    #[cfg(feature = "f32")]
    pub type Real = f32;

    /// The floating-point type used for all probabilistic computation.
    #[cfg(not(feature = "f32"))]
    pub type Real = f64;

    /// Mathematical constants (eg. `PI`) at [`Real`] precision.
    #[cfg(feature = "f32")]
    pub use std::f32::consts;

    /// Mathematical constants (eg. `PI`) at [`Real`] precision.
    #[cfg(not(feature = "f32"))]
    pub use std::f64::consts;
}

pub use real::Real;

///
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
pub fn logsumexp(xs: &Vec<Real>) -> Real {
    let max = xs.iter().cloned().fold(-1. / 0. /* -inf */, Real::max);
    if max == Real::NEG_INFINITY {
        Real::NEG_INFINITY
    } else {
        let mut sum_exp = 0.;
        for x in xs {
            sum_exp += (x - max).exp();
        }
        max + sum_exp.ln()
    }
}

// modeling libs
pub use address::{normalize_addr, AddrMap, SplitAddr};
pub use gfi::{ArgDiff, GenFn, Trace};
pub use modeling::dists::{
    bernoulli, beta, binomial, categorical, cauchy, dirichlet, exponential, gamma, geometric,
    inv_gamma, laplace, mvnormal, normal, poisson, u01, uniform, uniform_continuous,
    uniform_discrete, Distribution,
};
pub use modeling::dyngenfn::{DynGenFn, DynGenFnHandler};
pub use modeling::dyntrie::{
    dyn_debug_formatter, dyn_display_formatter, dyn_value_to_string, dyn_value_to_string_with,
    dyntrace_to_string, dyntrace_to_string_with, dyntrace_to_string_with_options,
    dyntrie_to_string, dyntrie_to_string_with, dyntrie_to_string_with_options, print_dyntrace,
    print_dyntrace_with, print_dyntrace_with_options, DynAutoCast, DynTrace, DynTracePrintOptions,
    DynTrie, DynValueFormatter,
};
pub use modeling::dynunfold::{DynParticles, DynUnfold};
pub use trie::Trie;

// inference libs
pub use inference::ParticleSystem;
pub use inference::{importance_resampling, importance_sampling};
pub use inference::{metropolis_hastings, mh, regen_mh, regenerative_metropolis_hastings};
