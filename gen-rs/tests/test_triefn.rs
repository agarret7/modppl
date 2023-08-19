use std::{any::Any, rc::Rc};
use gen_rs::{Trie, GenFn, TrieFn, TrieFnState, normal};


pub fn _triefn_prototype(state: &mut TrieFnState<f64,f64>,noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..3000).into_iter() {
        let x = state.sample_at(&normal, (1., noise), &format!("{}", i));
        sum += x;
    }
    sum
}

#[test]
pub fn test_triefn_prototype() {
    for _ in (0..100).into_iter() {
        let triefn_prototype = TrieFn::new(_triefn_prototype);

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