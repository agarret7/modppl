# modppl

[<img alt="crates.io" src="https://img.shields.io/crates/v/modppl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/modppl)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-modppl-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/modppl)
[<img alt="status" src="https://img.shields.io/github/actions/workflow/status/agarret7/modppl/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/agarret7/modppl/actions?query=branch%3Amain)

⚠ ️This is a research prototype, not intended for production use.

# What is modppl?

`modppl` is probabilistic programming written natively in Rust. Modeling and inference are separated via a trait interface called `GenFn`, whose implementations can simulate, generate, update, and regenerate `Trace` data structures, supporting composable inference kernels.


## Modeling & Inference

Probabilistic programming differs from deep learning in that it emphasizes developing inference algorithms rather than training weights. Different languages make different trade-offs in the inference algorithms they support, and designing structures that accommodate the widest possible set of Markovian processes is ongoing research.

Several inference procedures are supported:

- Importance Sampling and Resampling
- Proposal-based and Regenerative Metropolis-Hastings
- Sequential Monte Carlo (SMC)

ModPPL supports a compact, expressive, macro-based modeling DSL with the `dyngen!` macro that generates `Trace`s holding values of `Any` type at runtime. This includes support for `Unfold`, a sequential inference combinator. See [examples](https://github.com/agarret7/modppl/tree/main/modppl/tests/dyngenfns) for usage.

A robust observation model: most points are explained by tight noise, but outliers are drawn from a wide distribution.
```rust
use modppl::prelude::*;

dyngen!(
pub fn observe(mu: f64) -> f64 {
    let is_outlier = bernoulli(0.1) %= "is_outlier";
    let noise = if is_outlier { 5.0 } else { 0.5 };
    normal(mu, noise) %= "y"
});
```

Linear regression that traces each observation into a namespaced submodel via `/=`.
```rust
dyngen!(
pub fn linear_model(xs: Vec<f64>) -> Vec<f64> {
    let a = normal(0., 2.) %= "a";
    let b = normal(0., 2.) %= "b";
    xs.iter().enumerate().map(|(i, x)| {
        observe(a * x + b) /= &format!("obs/{}", i)
    }).collect()
});
```

A drift proposal perturbs the regression coefficients, leaving everything else fixed.
```rust
dyngen!(
pub fn drift_proposal(tr: Weak<DynTrace<Vec<f64>, Vec<f64>>>, drift: f64) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>("a"), drift) %= "a";
    normal(tr.data.read::<f64>("b"), drift) %= "b";
});
```

Composable inference alternating between drifting `a` and `b` and regenerating `outlier` labels.
```rust
let (mut trace, _) = linear_model.generate(xs.clone(), constraints);
for _ in 0..1000 {
    (trace, _) = mh(&linear_model, trace, &drift_proposal, 0.25);
    for i in 0..xs.len() {
        let mut mask = AddrMap::new();
        mask.visit(&format!("obs/{}/is_outlier", i));
        (trace, _) = regen_mh(&linear_model, trace, &mask);
    }
}
```

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
