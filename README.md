# gen-rs

[<img alt="github" src="https://img.shields.io/badge/agarret7/gen-rs?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/agarret7/gen-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gen-rs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gen-rs)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-gen_rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/gen-rs)
[<img alt="status" src="https://img.shields.io/github/actions/workflow/status/agarret7/gen-rs/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/agarret7/gen-rs/actions?query=branch%3Amain)

⚠ ️This is evolving software. Expect breaking API changes.

gen-rs is an experimental crate for probabilistic programming in Rust. gen-rs was inspired by [GenTL](https://github.com/OpenGen/GenTL/tree/main), but with Rust-native constructs.

It approximately* implements the Generative Function Interface [[GFI]](https://github.com/agarret7/gen-rs/blob/main/gen-rs/src/gfi.rs) as specified in the [Gen.jl whitepaper](https://dl.acm.org/doi/10.1145/3314221.3314642) and [Marco Cusumano-Towner's thesis](https://www.mct.dev/assets/mct-thesis.pdf). It provides several kinds of inference procedures built on this interface.


## Modeling

- Dynamically-typed `DynGenFn` and effects-based `DynGenFnHandler`
- `dyngen!` Dynamic Modeling Language
- Unfold Kernel Combinator
- [Example model implementations](https://github.com/agarret7/gen-rs/blob/main/gen-rs/tests/DynGenFns)


## Inference

- Importance Sampling
- Proposal-based MCMC
- Particle Filtering


## Gallery

Generate visualizations to `visualizations` with:
```shell
python -m venv venv && source venv/bin/activate && pip install matplotlib
cargo test && python visualization/visualizer.py
```


## Disclaimer

(*) `gen-rs` does not exactly implement the GFI. More precisely, `gen-rs` does not implement _retdiff_ or _choice gradients_.