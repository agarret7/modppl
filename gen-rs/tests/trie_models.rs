use std::any::Any;
use std::rc::Rc;
use gen_rs::{GenFn,Trie,modeling::triefn::{TrieFn,TrieFnState}, dists::{Distribution}};
use gen_rs::modeling::dists::{normal, categorical};
// use gen_rs::modeling::CallSite;
use gen_rs::{Trace, importance_sampling};
use rand::rngs::ThreadRng;


pub fn test_model(state: &mut TrieFnState<f64,f64>,noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..3000).into_iter() {
        let x = state.sample_at(&mut normal, (1., noise), Box::leak(format!("{}", i).into_boxed_str()));
        sum += x;
    }
    sum
}

#[test]
pub fn test_dynamic_model_prototype() {
    let mut dynamic_model_prototype = TrieFn::new(test_model);

    for i in (0..100).into_iter() {
        // let trace = dynamic_model_prototype.simulate(1.);
        let mut constraints = Trie::<Rc<dyn Any>>::new();
        constraints.insert_leaf_node("1", Rc::new(100.));
        constraints.insert_leaf_node("5", Rc::new(200.));
        let (trace, weight) = dynamic_model_prototype.generate(0.1, constraints);
        approx::assert_abs_diff_eq!(*trace.get_retv().unwrap(), 3298., epsilon = 50.);
        dbg!(trace.logpdf());
        dbg!(weight);
    }
}

fn _obs_model(state: &mut TrieFnState<(f64, f64, Vec<f64>),Vec<f64>>, args: (f64, f64, Vec<f64>)) -> Vec<f64> {
    let (slope, intercept, xs) = args;
    xs.into_iter()
        .enumerate()
        .map(|(i, x)| 
            state.sample_at(&mut normal, (slope * x + intercept, 0.1), Box::leak(format!("{}", i).into_boxed_str()))
        )
        .collect::<_>()
}

fn _line_model(state: &mut TrieFnState<Vec<f64>,Vec<f64>>, xs: Vec<f64>) -> Vec<f64> {
    let slope = state.sample_at(&mut normal, (0., 1.), "slope");
    let intercept = state.sample_at(&mut normal, (0., 2.), "intercept");
    state.trace_at(TrieFn::new(_obs_model), (slope, intercept, xs), "ys")
}

#[test]
pub fn test_bayesian_linear_regression() {
    const NUM_SAMPLES: u32 = 10000;

    let mut line_model = TrieFn::new(_line_model);
    let mut rng = ThreadRng::default();

    let xs = vec![-5., -4., -3., -2., -1., 0., 1., 2., 3., 4., 5.];
    let mut observations = Trie::new();
    xs.iter()
        .enumerate()
        .for_each(|(i, x)| {
            observations.insert_leaf_node(
                Box::leak(format!("ys => {}", i).into_boxed_str()),
                Rc::new(0.5*x - 1. + normal.random(&mut rng, (0., 0.1))) as Rc<dyn Any>);
            });
    let (traces, log_normalized_weights, lml_estimate) = importance_sampling(&mut line_model, xs, observations, NUM_SAMPLES);
    // dbg!(log_normalized_weights);

    let probs = log_normalized_weights.iter()
        .map(|w| w.exp())
        .collect::<Vec<f64>>();
    let traces = (0..NUM_SAMPLES/10)
        .map(|_| categorical.random(&mut rng, probs.clone()))
        .map(|idx| &traces[idx])
        .collect::<Vec<&Trace<_,_,_>>>();
    for i in 0..20 {
        println!("Trace {}", i);
        println!("slope = {}", &traces[i].data.get_leaf_node("slope").unwrap().clone().downcast::<f64>().ok().unwrap());
        println!("intercept = {}", &traces[i].data.get_leaf_node("intercept").unwrap().clone().downcast::<f64>().ok().unwrap());
    }
    dbg!(lml_estimate);
}