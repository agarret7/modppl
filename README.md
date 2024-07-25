# modppl

[<img alt="github" src="https://img.shields.io/badge/agarret7/modppl?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/agarret7/modppl)
[<img alt="crates.io" src="https://img.shields.io/crates/v/modppl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/modppl)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-modppl-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/modppl)
[<img alt="status" src="https://img.shields.io/github/actions/workflow/status/agarret7/modppl/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/agarret7/modppl/actions?query=branch%3Amain)

⚠ ️This is evolving software. The API may change.

# What is modppl?

`modppl` is probabilistic programming written natively in Rust. Modularity is conferred through a trait interface that separates modeling and inference, called `GenFn`.


## Inference

- Importance Sampling and Resampling
- Proposal-based and Regenerative Metropolis-Hastings
- Particle Filtering


## Dynamic Modeling

- Dynamically-typed `DynGenFn` and effects-based `DynGenFnHandler`
- `dyngen!` modeling language (sample with `%=`, trace with `/=`)
- Dynamic Unfold Kernel
- Check out some [examples](https://github.com/agarret7/modppl/tree/main/modppl/tests/dyngenfns)


## Gallery

Generate visualizations to `visualizations` with:
```shell
python -m venv venv && source venv/bin/activate && pip install matplotlib
cargo test --release && python visualization/visualizer.py
```


## Inspiration

`modppl` was inspired by the Generative Function Interface (GFI) as described in the Gen.jl whitepaper.

  Gen: A General-Purpose Probabilistic Programming System with Programmable Inference. Cusumano-Towner, M. F.; Saad, F. A.; Lew, A.; and Mansinghka, V. K. In Proceedings of the 40th ACM SIGPLAN Conference on Programming Language Design and Implementation (PLDI ‘19). ([pdf](https://dl.acm.org/doi/10.1145/3314221.3314642)) ([bibtex](https://www.gen.dev/assets/gen-pldi.txt)).

`modppl` does not exactly implement the GFI. More precisely, it does not support _retdiff_ or _choice gradients_.