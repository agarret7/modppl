use std::{any::Any, rc::Rc};
use gen_rs::{bernoulli, normal, uniform, GenFn, Trie, TrieFn, TrieFnState};


pub fn _triefn_prototype(state: &mut TrieFnState<f64,f64>,noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..3000).into_iter() {
        let x = state.sample_at(&normal, (1., noise), &format!("{}", i));
        sum += x;
    }
    sum
}
const triefn_prototype: TrieFn<f64,f64> = TrieFn { func: _triefn_prototype };

#[test]
pub fn test_triefn_prototype() {
    for _ in (0..100).into_iter() {
        let _trace = triefn_prototype.simulate(1.);
        let mut constraints = Trie::<Rc<dyn Any>>::new();
        constraints.insert_leaf_node("1", Rc::new(100.));
        constraints.insert_leaf_node("5", Rc::new(200.));
        let (trace, weight) = triefn_prototype.generate(0.1, Trie::from_unweighted(constraints));
        approx::assert_abs_diff_eq!(trace.retv.unwrap(), 3298., epsilon = 50.);
        dbg!(trace.logp);
        dbg!(weight);
    }
}


pub fn _triefn_sample_at_update_weight_regression(state: &mut TrieFnState<(),()>,_: ()) {
    let b = state.sample_at(&bernoulli, 0.25, "b");
    if b {
        state.sample_at(&normal, (0., 1.), "x");
    }
}
const triefn_sample_at_update_weight_regression: TrieFn<(),()> = TrieFn { func: _triefn_sample_at_update_weight_regression };

pub fn _triefn_trace_at_update_weight_regression(state: &mut TrieFnState<(),()>,_: ()) {
    let b = state.sample_at(&bernoulli, 0.25, "b");
    if b {
        println!("tracing at!");
        state.trace_at(&triefn_prototype, 1.0, "sub");
        println!("shoulda seen ")
    }
}
const triefn_trace_at_update_weight_regression: TrieFn<(),()> = TrieFn { func: _triefn_trace_at_update_weight_regression };

pub fn _triefn_sample_at_update_weight_regression2(state: &mut TrieFnState<(),()>,_: ()) {
    let m = state.sample_at(&uniform, (0.,1.), "m");
    state.sample_at(&normal, (m, 1.), "x");
    state.sample_at(&normal, (m, 1.), "y");
}
const triefn_sample_at_update_weight_regression2: TrieFn<(),()> = TrieFn { func: _triefn_sample_at_update_weight_regression2 };

#[test]
pub fn test_sample_at_update_prev_and_constrained() {
    // sample_at
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(true));
    constraints.insert_leaf_node("x", Rc::new(0.0));
    let tr = triefn_sample_at_update_weight_regression.generate((), Trie::from_unweighted(constraints)).0;
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("x", Rc::new(1.0));
    let w = triefn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, Trie::from_unweighted(constraints)).2;
    assert_eq!(w, -0.5);
}

#[test]
pub fn test_sample_at_update_no_prev_and_constrained() {
    // sample_at
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(false));
    let tr = triefn_sample_at_update_weight_regression.generate((), Trie::from_unweighted(constraints)).0;
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(true));
    constraints.insert_leaf_node("x", Rc::new(1.0));
    let w = triefn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, Trie::from_unweighted(constraints)).2;
    approx::assert_abs_diff_eq!(w, -2.517551, epsilon = 1e-6);
}

#[test]
pub fn test_update_sample_at_prev_and_unconstrained() {
    // sample_at
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("m", Rc::new(1.0));
    constraints.insert_leaf_node("x", Rc::new(1.0));
    constraints.insert_leaf_node("y", Rc::new(-0.3));
    let tr = triefn_sample_at_update_weight_regression2.generate((), Trie::from_unweighted(constraints)).0;
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("m", Rc::new(0.5));
    let w = triefn_sample_at_update_weight_regression2.update(tr, (), gen_rs::GfDiff::Unknown, Trie::from_unweighted(constraints)).2;
    approx::assert_abs_diff_eq!(w, 0.4000000, epsilon = 1e-6);
}

#[test]
pub fn test_update_no_prev_and_unconstrained() {
    // sample_at
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(false));
    let tr = triefn_sample_at_update_weight_regression.generate((), Trie::from_unweighted(constraints)).0;
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(true));
    let w = triefn_sample_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, Trie::from_unweighted(constraints)).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);

    // trace_at
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(false));
    let tr = triefn_trace_at_update_weight_regression.generate((), Trie::from_unweighted(constraints)).0;
    let mut constraints = Trie::<Rc<dyn Any>>::new();
    constraints.insert_leaf_node("b", Rc::new(true));
    let w = triefn_trace_at_update_weight_regression.update(tr, (), gen_rs::GfDiff::Unknown, Trie::from_unweighted(constraints)).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);
}