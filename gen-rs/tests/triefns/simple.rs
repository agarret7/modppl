use std::any::Any;
use std::rc::{Rc, Weak};
use gen_rs::{Trace,Trie,TrieFnState,TrieFn,normal,mvnormal};
use nalgebra::{DMatrix, DVector};

use crate::pointed_model;
use pointed_model::types_2d::{Bounds, Point, uniform_2d};


// bayesian linear regression model

fn _obs_model(state: &mut TrieFnState<(f64, f64, Vec<f64>),Vec<f64>>, args: (f64, f64, Vec<f64>)) -> Vec<f64> {
    let (slope, intercept, xs) = args;
    xs.into_iter()
        .enumerate()
        .map(|(i, x)| 
            state.sample_at(&normal, (slope * x + intercept, 0.1), &format!("{}", i))
        )
        .collect::<_>()
}
pub const obs_model: TrieFn<(f64, f64, Vec<f64>),Vec<f64>> = TrieFn { func: _obs_model };

fn _line_model(state: &mut TrieFnState<Vec<f64>,Vec<f64>>, xs: Vec<f64>) -> Vec<f64> {
    let slope = state.sample_at(&normal, (0., 1.), "slope");
    let intercept = state.sample_at(&normal, (0., 2.), "intercept");
    state.trace_at(&obs_model, (slope, intercept, xs), "ys")
}
pub const line_model: TrieFn<Vec<f64>,Vec<f64>> = TrieFn { func: _line_model };


// pointed model (TrieFn version)

fn _pointed_2d_model(state: &mut TrieFnState<(Bounds, DMatrix<f64>),Point>, args: (Bounds, DMatrix<f64>)) -> Point {
    let (bounds, cov) = args;
    let latent = state.sample_at(&uniform_2d, bounds, "latent");
    state.sample_at(&mvnormal, (latent, cov), "obs")
}
pub const pointed_2d_model: TrieFn<(Bounds,DMatrix<f64>),Point> = TrieFn { func: _pointed_2d_model };

// pointed proposal (TrieFn version)

fn _pointed_2d_drift_proposal(state: &mut TrieFnState<(Weak<Trace<(Bounds, DMatrix<f64>),Trie<(Rc<dyn Any>,f64)>,Point>>, DMatrix<f64>),()>,
                              args: (Weak<Trace<(Bounds, DMatrix<f64>),Trie<(Rc<dyn Any>,f64)>,Point>>, DMatrix<f64>)) -> () {
    let (trace, noise) = args;
    let trace = trace.upgrade().unwrap();
    let latent = trace.data.get_leaf_node("latent").unwrap().clone().0.downcast::<DVector<f64>>().ok().unwrap();
    state.sample_at(&mvnormal, (latent.as_ref().clone(), noise), "latent");
}
pub const pointed_2d_drift_proposal: TrieFn<(Weak<Trace<(Bounds, DMatrix<f64>),Trie<(Rc<dyn Any>,f64)>,Point>>,DMatrix<f64>),()> = TrieFn { func: _pointed_2d_drift_proposal };