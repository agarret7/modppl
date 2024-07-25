# Change Log

Changes to `modppl` starting with `v0.3.0` are documented here.

## [0.3.0]

### Modified

- Renamed crate from `gen-rs` to `modppl`
- Several major breaking API changes in `Trie`. The updates are roughly:
  - `Trie` has a new simplified API:
    - `Trie::new()` returns an empty `Trie`.
    - `Trie::leaf(value: V, weight: f64)` returns a `Trie<V>` with an inner value and weight.
      - Value via `ref_inner`, `take_inner`, `replace_inner`, and `expect_inner`.
    - `trie.weight()` returns the weight.
    - `trie.search(addr: &str) -> Option<&Trie<V>>` returns `Some(sub)` at `addr` if occupied, otherwise None.
    - `trie.observe(addr: &str, value: V)` inserts an unweighted leaf at a valid unoccupied `addr` (panics if occupied).
    - `trie.w_observe(addr: &str, value: V, weight: f64)` inserts a weighted leaf at a valid unoccupied `addr` (panics if occupied) and increases the `trie`'s weight by `weight`.
    - `trie.insert(addr: &str, sub: Trie<V>)` inserts a sub `Trie` at a valid unoccupied `addr` (panics if occupied) and increases the `trie`'s weight by `sub.weight()`.
    - `trie.remove(addr: &str) -> Option<Trie<V>>` removes and returns `Some(sub)` at `addr` if occupied, decreasing `trie`'s weight by `sub.weight()`, otherwise does nothing and returns `None`.
    - `trie.merge(other: Trie<V>)` consumes another `Trie`, merging it into `self` preferentially using the values of `other` at overlapping addresses.
    - `trie.schema()` returns an `AddrMap` representing the address schema of `self`.
    - `trie.collect(mask: &AddrMap)` collects the set of values identified by `mask` into a new `Trie`, leaving values in `self` that are in the complement of `mask`. Returns the new `self`, the collected value trie, and the weight of the collected value trie.
  - `TrieFn` was renamed to `DynGenFn`. Correspondingly:
    - `TrieFnState` => `DynGenFnHandler`
    - `Trie<(Rc<dyn Any>>,f64)>` => `DynTrie` := `Trie<Arc<dyn Any + Send + Sync>>`
    - `Unfold` => `DynUnfold` and now takes in a `DynGenFn` instead of a raw `fn` item
- `Trace::logp` => `Trace::logjp`
- `Rc` => `Arc` in `DynTrie` and across the codebase.
- `GfDiff` => `ArgDiff`
- `categorical` returns an `i64` instead of `usize`


### Added

- Start of semver
- Prelude file (`use modppl::prelude::*;`)
- `modppl-macros`, providing `dyngen!` to automatically implement and instantiate `DynGenFn` from `fn` items
- `AddrMap`: a recursive `HashMap` serving as a mask
- optional `regenerate`
```rust
    fn regenerate(&self,
        trace: Trace<Args,Data,Ret>,
        args: Args,
        diff: ArgDiff,
        mask: &AddrMap
    ) -> (Trace<Args,Data,Ret>, f64);
```
- `importance_resampling`
- `regenerative_metropolis_hastings` (alias `regen_mh`, both available if `regenerate` is implemented)
- `DynParticles<State> := ParticleSystem<State,Vec<DynTrie>,Vec<State>,DynUnfold<State>>`
- `geometric` distribution (returns `i64`)
- `poisson` distribution (returns `i64`)
- `beta` distribution (returns `f64`)
- `gamma` distribution (returns `f64`)


### Removed

- `GLOBAL_RNG`
- `AddrTrie`