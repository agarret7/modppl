# gen-rs

[<img alt="github" src="https://img.shields.io/badge/agarret7/gen-rs?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/agarret7/gen-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gen-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gen-rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-gen_rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/gen-rs)
[<img alt="status" src="https://img.shields.io/github/actions/workflow/status/agarret7/gen-rs/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/agarret7/gen-rs/actions?query=branch%3Amain)

gen-rs is an experimental, research crate for probabilistic programming in Rust. gen-rs supports many features of other Rust-native probabilistic frameworks such as [Ferric](https://github.com/ferric-ai/ferric), and is written at a lower-level than languages such as [Gen.jl](https://github.com/probcomp/Gen.jl). gen-rs was inspired by [GenTL](https://github.com/OpenGen/GenTL/tree/main), but with Rust-native constructs.

It implements the Generative Function Interface [[GFI]](https://github.com/agarret7/gen-rs/blob/main/gen-rs/src/gfi.rs) as specified in the [Gen.jl whitepaper](https://dl.acm.org/doi/10.1145/3314221.3314642) and [Marco Cusumano-Towner's thesis](https://www.mct.dev/assets/mct-thesis.pdf)) and a basic, yet complete set of inference procedures.


## Modeling Features

- Generative Function Interface (GFI) compatible
- Dynamically-typed `TrieFn` DSL
- Unfold with incremental computation
- [Example model implementations](https://github.com/agarret7/gen-rs/blob/main/gen-rs/tests/triefns)


## Inference Features

- Importance Sampling
- Proposal-based MCMC
- Particle Filtering

Generate visualizations to `visualizations` with:
```shell
python -m venv venv && source venv/bin/activate && pip install matplotlib
cargo test && python visualization/visualizer.py
```


## Disclaimer

Unlike most modern ML systems, probabilistic programming doesn't require a differentiable likelihood. A fast (possibly parallelized) CPU-bound iterator is often sufficient for inference. This aligns well with Rust's principle of "fearless concurrency". AD support is not currently planned for this library.