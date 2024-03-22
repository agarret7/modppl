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

#[macro_use]
extern crate serde_derive;


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

/// Implementations of the `Trie` data structure, used extensively in `modeling::DynGenFn`. 
// pub mod trie;

pub mod trie;

/// Distributions and a modeling DSL built on `Trie`s.
pub mod modeling;

/// Standard inference library.
pub mod inference;

mod mathutils;

// modeling libs
// pub use trie::Trie;
pub use trie::{Trie,AddrTrie};
pub use address::{SplitAddr, normalize_addr};
pub use gfi::{Trace, GenFn, GfDiff};
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
pub use modeling::dyngenfn::{DynGenFn,DynGenFnHandler,DynTrie};
pub use modeling::unfold::Unfold;
pub use mathutils::logsumexp;

// inference libs
pub use inference::importance_sampling;
pub use inference::metropolis_hastings;
pub use inference::mh;
pub use inference::ParticleSystem;

// // Macro to implement Distribution with custom logic and automatically register the implementation
// macro_rules! register_distribution {
//     ($t:ty, $impl:block) => {
//         impl Distribution for $t {
//             $impl
//         }

//         // This part ensures that the type and its Distribution implementation are registered
//         // You might want to adjust when and how this registration happens based on your application's needs
//         {
//             let instance = <$t>::default();
//             let mut registry = REGISTRY.lock().unwrap();
//             registry.insert(stringify!($t).to_string(), Box::new(instance));
//         }
//     };
// }

// // Example usage of the macro with a custom implementation for MyDistribution
// #[derive(Default)]
// struct MyDistribution;

// register_distribution!(MyDistribution, {
//     fn sample(&self) -> i32 {
//         // Custom implementation for MyDistribution
//         100 // Just a placeholder value
//     }
// });

// // Another example with a different distribution
// #[derive(Default)]
// struct AnotherDistribution;

// register_distribution!(AnotherDistribution, {
//     fn sample(&self) -> i32 {
//         // Custom implementation for AnotherDistribution
//         200 // Just a placeholder value
//     }
// });

// extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, ItemStruct};

// #[proc_macro_attribute]
// pub fn register_distribution(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemStruct);
//     let name = &input.ident;

//     // You could allow attributes to specify details for the registration, handled here:
//     // let _ = parse_macro_input!(attr as AttributeArgs);

//     // Generate the implementation of the Distribution trait and registration logic
//     let expanded = quote! {
//         #input

//         impl Distribution for #name {
//             fn sample(&self) -> i32 {
//                 // The user would implement this method's body directly in the struct definition
//                 unimplemented!()
//             }
//         }

//         // Assuming a function `register_distribution_impl` exists and handles the actual registration
//         register_distribution_impl(stringify!(#name), Box::new(#name::default()));
//     };

//     TokenStream::from(expanded)
// }