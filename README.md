# genark

This library contains highly-experimental explorations of probabilistic programming interfaces (and in particular the Generative Function Interface as specified in Marco Cusumano-Towner's thesis) via "self-specializing SMC" in Rust. This was heavily inspired by much more fully-featured projects in the OpenGen ecosystem such as Gen.jl, GenParticleFilters, SMCP3, 3DP3, GenJax, and GenTL. You should probably prefer one of those (unless smote and/or tail re-cursed).


## Modeling and Inference Gallery

TBD


## Motivation

As it stands, the OpenGen ecosystem is mostly leveraged by scientific practitioners in Julia or advanced users of Google's Jax. I believe it's plausible that Rust could one day expand the scope of OpenGen to a broader community of hard-working and dedicated open-source systems engineers.

Note unlike most modern ML systems, probabilistic programming doesn't require a differentiable likelihood; a fast (possibly parallelized) iterator is often sufficient for inference. This aligns well with Rust's principle of "fearless concurrency". However, most embodied (read: practical) modeling efforts will still require parameter tuning and Langevin or Hamiltonian Monte Carlo inference moves, to effectively leverage numerical gradients of the local energy landscape in top-down or supervised data processing.

AD support in Rust is currently quite shaky, limiting these applications.


## Applications

As of 2023, personal physiological tracking and cybernetics. I'd love to see security-conscious applications like autonomous vehicles or bioinformatics leverage Rust's ownership system and memory-safety properties to deploy larger-scale and high-throughput statistical pipelines, but that's more speculation than reality at this stage.
