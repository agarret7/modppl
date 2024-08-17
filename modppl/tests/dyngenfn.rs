use modppl::prelude::*;

mod pointed_model;
mod dyngenfns;


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
        dbg!(trace.logjp);
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
        state.trace_at(&DynGenFn_prototype, 1.0, "sub");
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
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), modppl::ArgDiff::Unknown, constraints).2;
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
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), modppl::ArgDiff::Unknown, constraints).2;
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
    let w = DynGenFn_sample_at_update_weight_regression2.update(tr, (), modppl::ArgDiff::Unknown, constraints).2;
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
    let w = DynGenFn_sample_at_update_weight_regression.update(tr, (), modppl::ArgDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);

    // trace_at
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(false));
    let tr = DynGenFn_trace_at_update_weight_regression.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("b", Arc::new(true));
    let w = DynGenFn_trace_at_update_weight_regression.update(tr, (), modppl::ArgDiff::Unknown, constraints).2;
    approx::assert_abs_diff_eq!(w, -1.098612, epsilon = 1e-6);
}

#[test]
#[should_panic]
pub fn test_generate_residual_constraints_panic() {
    let mut constraints = DynTrie::new();
    constraints.observe("abc", Arc::new(0.));
    DynGenFn_prototype.generate(0.1, constraints);
}

#[test]
#[should_panic]
pub fn test_update_residual_constraints_panic() {
    let mut constraints = DynTrie::new();
    constraints.observe("abc", Arc::new(0.));
    let trace = DynGenFn_prototype.simulate(0.1);
    DynGenFn_prototype.update(trace, 0.1, ArgDiff::NoChange, constraints);
}

dyngen!(
fn hyperprior(a: f64, b: f64) -> bool {
    let p = beta(a,b) %= "prob_is_small";
    bernoulli(p) %= "is_small"
});

dyngen!(
fn model() -> f64 {
    if hyperprior(2.,2.) /= "var" {
        normal(0.,0.05) %= "y"
    } else {
        normal(0.,1.0) %= "y"
    }
});

dyngen!(
fn proposal(tr: Weak<DynTrace<(),f64>>, drift: f64, addr: String) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>(&addr), drift) %= &addr;
});

#[test]
pub fn test_parse() {
    let mut constraints = DynTrie::new();
    constraints.observe("y", Arc::new(0.3));
    let mut tr = model.simulate(());
    for _ in 0..1000 {
        let (new_tr, accepted) = mh(&model, tr, &proposal, (0.5,String::from("var/prob_is_small")));
        dbg!(accepted);
        tr = new_tr;
    }
}

#[test]
pub fn test_simulate() {
    dyngen!(
    fn foo(p: f64) -> bool {
        bernoulli(p) %= "x"
    });

    let p = 0.4;
    let trace = foo.simulate(p);
    assert_eq!(trace.data.read::<bool>("x"), trace.retv.unwrap());
    assert_eq!(trace.args, p);
    assert_eq!(trace.logjp, if trace.data.read::<bool>("x") { p.ln() } else { (1.-p).ln() });
}

#[test]
pub fn test_update() {
    dyngen!(
    fn bar() -> f64 {
        normal(0., 1.) %= "a"
    });

    dyngen!(
    fn baz() -> f64 {
        normal(0., 1.) %= "b"
    });

    dyngen!(
    fn foo() -> f64 {
        if bernoulli(0.4) %= "branch" {
            normal(0., 1.) %= "x";
            bar() /= "u"
        } else {
            normal(0., 1.) %= "y";
            baz() /= "v"
        }
    });

    // get a trace which follows the first branch
    let mut constraints = DynTrie::new();
    constraints.observe("branch", Arc::new(true));
    let (trace, _) = foo.generate((), constraints);
    let x = trace.data.read::<f64>("x");
    let a = trace.data.read::<f64>("u/a");

    // force to follow the second branch
    let y = 1.123;
    let b = -2.1;
    let mut constraints = DynTrie::new();
    constraints.observe("branch", Arc::new(false));
    constraints.observe("y", Arc::new(y));
    constraints.observe("v/b", Arc::new(b));
    let (new_trace, discard, weight) = foo.update(trace, (), ArgDiff::NoChange, constraints);

    // test discard
    assert_eq!(discard.read::<bool>("branch"), true);
    assert_eq!(discard.read::<f64>("x"), x);
    assert_eq!(discard.read::<f64>("u/a"), a);
    assert_eq!(discard.iter().fold(0, |l, (_, tr)| l + tr.is_leaf() as usize), 2);
    assert_eq!(discard.iter().fold(0, |l, (_, tr)| l + !tr.is_leaf() as usize), 1);

    // test new trace
    let new_assignment = new_trace.data;
    assert_eq!(new_assignment.read::<bool>("branch"), false);
    assert_eq!(new_assignment.read::<f64>("y"), y);
    assert_eq!(new_assignment.read::<f64>("v/b"), b);
    assert_eq!(new_assignment.iter().fold(0, |l, (_, tr)| l + tr.is_leaf() as usize), 2);
    assert_eq!(new_assignment.iter().fold(0, |l, (_, tr)| l + !tr.is_leaf() as usize), 1);

    // test logjp and weight
    let prev_logjp =
        bernoulli.logpdf(&true, 0.4) +
        normal.logpdf(&x, (0., 1.)) +
        normal.logpdf(&a, (0., 1.));
    let expected_new_logjp =
        bernoulli.logpdf(&false, 0.4) +
        normal.logpdf(&y, (0., 1.)) +
        normal.logpdf(&b, (0., 1.));
    let expected_weight = expected_new_logjp - prev_logjp;
    approx::assert_abs_diff_eq!(expected_new_logjp, new_trace.logjp, epsilon = 1e-3);
    approx::assert_abs_diff_eq!(expected_weight, weight, epsilon = 1e-3);

    // addresses under the "data" namespace will be visited,
    // but nothing there will be discarded.
    dyngen!(
    fn loopy() {
        let a = normal(0., 1.) %= "a";
        for i in 0..5 {
            normal(a, 1.) %= &format!("data/{i}");
        }
    });

    // get an initial trace
    let mut constraints = DynTrie::new();
    constraints.observe("a", Arc::new(0.));
    for i in 0..5 {
        constraints.observe(&format!("data/{i}"), Arc::new(0.));
    }
    let (trace, _) = loopy.generate((), constraints);

    // update "a"
    let mut constraints = DynTrie::new();
    constraints.observe("a", Arc::new(1.));
    let (new_trace, discard, weight) = loopy.update(trace, (), ArgDiff::NoChange, constraints);

    // test discard, logjp, weight
    assert_eq!(discard.read::<f64>("a"), 0.);
    let prev_logjp = 6. * normal.logpdf(&0., (0., 1.));
    let expected_new_logjp = normal.logpdf(&1., (0., 1.)) + 5. * normal.logpdf(&0., (1., 1.));
    let expected_weight = expected_new_logjp - prev_logjp;
    approx::assert_abs_diff_eq!(expected_new_logjp, new_trace.logjp, epsilon = 1e-3);
    approx::assert_abs_diff_eq!(expected_weight, weight, epsilon = 1e-3);

    dyngen!(
    fn hierarchical_update() {
        let k = poisson(5.) %= "k";
        for i in 0..k {
            uniform(0.,1.) %= &format!("value/{i}");
        }
    });

    let mut constraints = DynTrie::new();
    constraints.observe("k", Arc::new(3_i64));
    let trace = hierarchical_update.generate((), constraints).0;
    let mut constraints = DynTrie::new();
    constraints.observe("k", Arc::new(1_i64));
    let (_, discard, weight) = hierarchical_update.update(trace, (), ArgDiff::Unknown, constraints);
    assert!(discard.search("value/1").is_some());
    assert!(discard.search("value/2").is_some());
    assert_eq!(
        weight,
        poisson.logpdf(&1, 5.) - 
        poisson.logpdf(&3, 5.) -
        uniform.logpdf(&0.5, (0., 1.)) -
        uniform.logpdf(&0.5, (0., 1.))
    );
}

#[test]
pub fn test_regenerate() {
    dyngen!(
    fn bar(mu: f64) -> f64 {
        normal(mu, 1.) %= "a"
    });

    dyngen!(
    fn baz(mu: f64) -> f64 {
        normal(mu, 1.) %= "b"
    });

    dyngen!(
    fn foo(mu: f64) -> f64 {
        if bernoulli(0.4) %= "branch" {
            normal(mu, 1.) %= "x";
            bar(mu) /= "u"
        } else {
            normal(mu, 1.) %= "y";
            baz(mu) /= "v"
        }
    });

    // get a trace which follows the first branch
    let mut mu = 0.123;
    let mut constraints = DynTrie::new();
    constraints.observe("branch", Arc::new(true));
    let (mut trace, _) = foo.generate(mu, constraints);
    let x = trace.data.read::<f64>("x");
    let a = trace.data.read::<f64>("u/a");

    let mut mask = AddrMap::new();
    mask.visit("branch");

    // change the argument so that the weights can be nonzero
    let mut rng = ThreadRng::default();
    for i in 0..10 {
        let prev_branch = trace.data.read::<bool>("branch");

        // test logjp
        let prev_mu = mu;
        mu = u01(&mut rng);
        let (new_trace, weight) = foo.regenerate(trace, mu, ArgDiff::Unknown, &mask);
        trace = new_trace;

        // test logjp
        let expected_logjp = if trace.data.read::<bool>("branch") {
            normal.logpdf(&trace.data.read::<f64>("x"), (mu, 1.)) +
            normal.logpdf(&trace.data.read::<f64>("u/a"), (mu, 1.)) +
            bernoulli.logpdf(&true, 0.4)
        } else {
            normal.logpdf(&trace.data.read::<f64>("y"), (mu, 1.)) +
            normal.logpdf(&trace.data.read::<f64>("v/b"), (mu, 1.)) +
            bernoulli.logpdf(&false, 0.4)
        };
        approx::assert_abs_diff_eq!(expected_logjp, trace.logjp, epsilon = 1e-3);

        // test values
        if trace.data.read::<bool>("branch") {
            assert!(trace.data.search("x").is_some());
            assert!(!trace.data.search("u").unwrap().is_leaf());
        } else {
            assert!(trace.data.search("y").is_some());
            assert!(!trace.data.search("v").unwrap().is_leaf());
        }
        assert_eq!(trace.data.iter().fold(0, |l, (_, tr)| l + tr.is_leaf() as usize), 2);
        assert_eq!(trace.data.iter().fold(0, |l, (_, tr)| l + !tr.is_leaf() as usize), 1);

        // test weight
        let mut expected_weight = 0.;
        if trace.data.read::<bool>("branch") == prev_branch {
            expected_weight = if trace.data.read::<bool>("branch") {
                normal.logpdf(&trace.data.read::<f64>("x"), (mu, 1.)) +
                normal.logpdf(&trace.data.read::<f64>("u/a"), (mu, 1.)) -
                normal.logpdf(&trace.data.read::<f64>("x"), (prev_mu, 1.)) -
                normal.logpdf(&trace.data.read::<f64>("u/a"), (prev_mu, 1.))
            } else {
                normal.logpdf(&trace.data.read::<f64>("y"), (mu, 1.)) +
                normal.logpdf(&trace.data.read::<f64>("v/b"), (mu, 1.)) -
                normal.logpdf(&trace.data.read::<f64>("y"), (prev_mu, 1.)) -
                normal.logpdf(&trace.data.read::<f64>("v/b"), (prev_mu, 1.))
            }
        }
        approx::assert_abs_diff_eq!(expected_weight, weight, epsilon = 1e-3);
    }
}
