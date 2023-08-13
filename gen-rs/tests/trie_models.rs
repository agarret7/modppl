use std::any::Any;
use std::fs::write;
use std::rc::Rc;
use gen_rs::{GenFn,Trie,modeling::triefn::{TrieFnState, TrieFn}, dists::{Distribution, mvnormal}};
use gen_rs::modeling::dists::{normal, categorical};
use gen_rs::{Trace, importance_sampling};
use nalgebra::{DMatrix, DVector, dmatrix, dvector};
use rand::rngs::ThreadRng;


pub fn _dynamic_model_prototype(state: &mut TrieFnState<f64,f64>,noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..3000).into_iter() {
        let x = state.sample_at(&normal, (1., noise), &format!("{}", i));
        sum += x;
    }
    sum
}

#[test]
pub fn test_dynamic_model_prototype() {

    for _ in (0..100).into_iter() {
        let mut dynamic_model_prototype = TrieFn::new(_dynamic_model_prototype);

        let _trace = dynamic_model_prototype.simulate(1.);
        let mut constraints = Trie::<Rc<dyn Any>>::new();
        constraints.insert_leaf_node("1", Rc::new(100.));
        constraints.insert_leaf_node("5", Rc::new(200.));
        let (trace, weight) = dynamic_model_prototype.generate(0.1, Trie::from_unweighted(constraints));
        approx::assert_abs_diff_eq!(trace.retv.unwrap(), 3298., epsilon = 50.);
        dbg!(trace.logp);
        dbg!(weight);
    }
}

fn _obs_model(state: &mut TrieFnState<(f64, f64, Vec<f64>),Vec<f64>>, args: (f64, f64, Vec<f64>)) -> Vec<f64> {
    let (slope, intercept, xs) = args;
    xs.into_iter()
        .enumerate()
        .map(|(i, x)| 
            state.sample_at(&normal, (slope * x + intercept, 0.1), &format!("{}", i))
        )
        .collect::<_>()
}

fn _line_model(state: &mut TrieFnState<Vec<f64>,Vec<f64>>, xs: Vec<f64>) -> Vec<f64> {
    let slope = state.sample_at(&normal, (0., 1.), "slope");
    let intercept = state.sample_at(&normal, (0., 2.), "intercept");
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
    let (traces, log_normalized_weights, lml_estimate) = importance_sampling(&mut line_model, xs, Trie::from_unweighted(observations), NUM_SAMPLES);
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
        println!("slope = {}", &traces[i].data.get_leaf_node("slope").unwrap().0.clone().downcast::<f64>().ok().unwrap());
        println!("intercept = {}", &traces[i].data.get_leaf_node("intercept").unwrap().0.clone().downcast::<f64>().ok().unwrap());
    }
    dbg!(lml_estimate);
}


mod pointed;
use pointed::types_2d::{Bounds, Point, uniform_2d};

fn _pointed_2d_model(state: &mut TrieFnState<(Bounds, DMatrix<f64>),Point>, args: (Bounds, DMatrix<f64>)) -> Point {
    let (bounds, cov) = args;
    let latent = state.sample_at(&uniform_2d, bounds, "latent");
    state.sample_at(&mvnormal, (latent, cov), "obs")
}

use std::rc::Weak;

fn _pointed_2d_drift_proposal(state: &mut TrieFnState<(Weak<Trace<(Bounds, DMatrix<f64>),Trie<(Rc<dyn Any>,f64)>,Point>>, DMatrix<f64>),()>,
                              args: (Weak<Trace<(Bounds, DMatrix<f64>),Trie<(Rc<dyn Any>,f64)>,Point>>, DMatrix<f64>)) -> () {
    let (trace, noise) = args;
    let trace = trace.upgrade().unwrap();
    let latent = trace.data.get_leaf_node("latent").unwrap().clone().0.downcast::<DVector<f64>>().ok().unwrap();
    state.sample_at(&mvnormal, (latent.as_ref().clone(), noise), "latent");
}

use gen_rs::inference::metropolis_hastings;

#[test]
pub fn test_mcmc() -> std::io::Result<()>{
    const NUM_ITERS: u32 = 25000;

    let mut model = TrieFn::new(_pointed_2d_model);
    let mut proposal = TrieFn::new(_pointed_2d_drift_proposal);

    let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
    let obs = dvector![0., 0.];

    let mut observations = Trie::new();
    observations.insert_leaf_node("obs", Rc::new(obs) as Rc<dyn Any>);
    let observations = Trie::from_unweighted(observations);

    let mut trace = model.generate((bounds, dmatrix![1., -3./5.; -3./5., 2.]), observations).0;
    for iter in 0..NUM_ITERS {
        dbg!(iter);
        let (new_trace, accepted) = metropolis_hastings(&mut model, trace, &mut proposal, dmatrix![0.25, 0.; 0., 0.25]);
        dbg!(accepted);
        trace = new_trace;
        let data = trace.data.get_leaf_node("latent").unwrap().0.clone().downcast::<DVector<f64>>().ok().unwrap();
        let json = format!("[{},{}]", data[0], data[1]);
        write(format!("../data/mh_trace_{}.json", iter), json)?;
    }

    Ok(())
}