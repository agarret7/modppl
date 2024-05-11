use std::{any::Any, sync::Arc};
use gen_rs::{bernoulli, normal, uniform, GenFn, DynTrie, DynGenFn, DynGenFnHandler};


pub fn _DynGenFn_prototype(state: &mut DynGenFnHandler<f64,f64>,noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..3000).into_iter() {
        let x = state.sample_at(&normal, (1., noise), &format!("{}", i));
        sum += x;
    }
    sum
}
const DynGenFn_prototype: DynGenFn<f64,f64> = DynGenFn { func: _DynGenFn_prototype };

#[test]
pub fn test_DynGenFn_prototype() {
    for _ in (0..100).into_iter() {
        let _trace = DynGenFn_prototype.simulate(1.);
        let mut constraints = DynTrie::new();
        constraints.observe("1", Arc::new(100.));
        constraints.observe("5", Arc::new(200.));
        let (trace, weight) = DynGenFn_prototype.generate(0.1, constraints);
        approx::assert_abs_diff_eq!(trace.retv.unwrap(), 3298., epsilon = 50.);
        dbg!(trace.logp);
        dbg!(weight);
    }
}


pub fn _DynGenFn_sample_at_update_weight_regression(state: &mut DynGenFnHandler<(),()>,_: ()) {
    let b = state.sample_at(&bernoulli, 0.25, "b");
    if b {
        state.sample_at(&normal, (0., 1.), "x");
    }
}
const DynGenFn_sample_at_update_weight_regression: DynGenFn<(),()> = DynGenFn { func: _DynGenFn_sample_at_update_weight_regression };

pub fn _DynGenFn_trace_at_update_weight_regression(state: &mut DynGenFnHandler<(),()>,_: ()) {
    let b = state.sample_at(&bernoulli, 0.25, "b");
    if b {
        println!("tracing at!");
        state.trace_at(&DynGenFn_prototype, 1.0, "sub");
        println!("shoulda seen ")
    }
}
const DynGenFn_trace_at_update_weight_regression: DynGenFn<(),()> = DynGenFn { func: _DynGenFn_trace_at_update_weight_regression };

pub fn _DynGenFn_sample_at_update_weight_regression2(state: &mut DynGenFnHandler<(),()>,_: ()) {
    let m = state.sample_at(&uniform, (0.,1.), "m");
    state.sample_at(&normal, (m, 1.), "x");
    state.sample_at(&normal, (m, 1.), "y");
}
const DynGenFn_sample_at_update_weight_regression2: DynGenFn<(),()> = DynGenFn { func: _DynGenFn_sample_at_update_weight_regression2 };

#[test]
pub fn test_sample_at_update_prev_and_constrained() {
    // sample_at
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(true));
    constraints.observe("x", Arc::new(0.0));
    let tr = DynGenFn_sample_at_update_weight_regression.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("x", Arc::new(1.0));
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, constraints).2;
    assert_eq!(w, -0.5);
}

#[test]
pub fn test_sample_at_update_no_prev_and_constrained() {
    // sample_at
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(false));
    let tr = DynGenFn_sample_at_update_weight_regression.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(true));
    constraints.observe("x", Arc::new(1.0));
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, -2.517551, epsilon = 1e-6);
}

#[test]
pub fn test_update_sample_at_prev_and_unconstrained() {
    // sample_at
    let mut constraints = DynTrie::new();
    constraints.observe("m", Arc::new(1.0));
    constraints.observe("x", Arc::new(1.0));
    constraints.observe("y", Arc::new(-0.3));
    let tr = DynGenFn_sample_at_update_weight_regression2.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("m", Arc::new(0.5));
    let w = DynGenFn_sample_at_update_weight_regression2.update(tr, (), gen_rs::GfDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, 0.4000000, epsilon = 1e-6);
}

#[test]
pub fn test_update_no_prev_and_unconstrained() {
    // sample_at
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(false));
    let tr = DynGenFn_sample_at_update_weight_regression.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(true));
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);

    // trace_at
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(false));
    let tr = DynGenFn_trace_at_update_weight_regression.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(true));
    let w = DynGenFn_trace_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);
}