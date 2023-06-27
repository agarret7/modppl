# gen-rs

This library contains highly-experimental explorations of probabilistic programming interfaces (and in particular the Generative Function Interface as specified in Marco Cusumano-Towner's thesis) via "self-specializing SMC" in Rust. This was heavily inspired by GenTL, and much more fully-featured projects in the OpenGen ecosystem such as Gen.jl, GenParticleFilters, SMCP3, 3DP3, and GenJax. You should probably prefer one of those (unless tail re-cursed).


## Modeling and Inference Gallery

TBD


## Motivation

Primarily self-edification.

Unlike most modern ML systems, probabilistic programming doesn't require a differentiable likelihood; a fast (possibly parallelized) iterator is often sufficient for inference. This aligns well with Rust's principle of "fearless concurrency". However, most embodied (read: practical) modeling efforts still require extensive parameter tuning and Langevin or Hamiltonian Monte Carlo inference moves, to effectively leverage numerical gradients of the local energy landscape in top-down or supervised data processing.

Despite Rust being an absolutely delightful experience to program in, AD support and GPU acceleration is somewhat shaky (given the lack of first-class Rust-native tensor libraries), limiting these applications.


## Applications

As of 2023, personal physiological tracking and cybernetics. I'd love to see security-conscious applications like autonomous vehicles or bioinformatics leverage Rust's ownership system and memory-safety properties to deploy larger-scale and high-throughput statistical pipelines, but that's total speculation at this stage.

Furthermore, since large-scale open-source simulation is still firmly rooted in the C/C++ (and to a lesser extent C#) world, I don't expect this to change anytime soon.