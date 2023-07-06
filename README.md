# gen-rs

![status](https://github.com/agarret7/gen-rs/actions/workflows/test.yml/badge.svg)

This library contains highly-experimental explorations of probabilistic programming via "self-specializing SMC" in Rust (and in particular a few reference implementations of models implementing the Generative Function Interface [[GFI]](https://github.com/agarret7/gen-rs/blob/main/gen-rs/src/gfi.rs) as specified in the [Gen.jl whitepaper](https://dl.acm.org/doi/10.1145/3314221.3314642) and [Marco Cusumano-Towner's thesis](https://www.mct.dev/assets/mct-thesis.pdf)).


## Inference Gallery

Generate visualizations with:
```shell
python -m venv venv && activate venv/bin/activate && pip install matplotlib
cargo test && python visualization/visualizer.py
```


## Disclaimer

Unlike most modern ML systems, probabilistic programming doesn't require a differentiable likelihood; a fast (possibly parallelized) CPU-bound iterator is often sufficient for inference. This aligns well with Rust's principle of "fearless concurrency". However, most embodied (read: practical) modeling efforts still require extensive parameter tuning and Langevin or Hamiltonian Monte Carlo inference moves, to effectively leverage numerical gradients of the local energy landscape in top-down or supervised data processing.

Despite Rust being a delightful experience to program in, AD support and GPU acceleration is still somewhat shaky (given the lack of first-class Rust-native tensor libraries), limiting these applications.

This project was heavily inspired by [GenTL](https://github.com/OpenGen/GenTL/tree/main) and several more fully-featured projects in the OpenGen ecosystem such as [Gen.jl](https://github.com/probcomp/Gen.jl/tree/master), [GenParticleFilters](https://github.com/probcomp/GenParticleFilters.jl), [SMCP3](https://github.com/probcomp/aistats2023-smcp3), [Bayes3D](https://github.com/probcomp/bayes3d/tree/main), and GenJax. It's recommended you check out one of those before deciding whether or not to use this package.