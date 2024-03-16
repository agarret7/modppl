use std::any::Any;
use std::rc::{Rc, Weak};
use gen_rs::{Trace,DynTrie,DynGenFnHandler,DynGenFn,normal,mvnormal};
use nalgebra::{DMatrix, DVector};

use crate::pointed_model;
use pointed_model::types_2d::{Bounds, Point, uniform_2d};


// bayesian linear regression model

fn _obs_model(state: &mut DynGenFnHandler<(f64, f64, Vec<f64>),Vec<f64>>, args: (f64, f64, Vec<f64>)) -> Vec<f64> {
    let (slope, intercept, xs) = args;
    xs.into_iter()
        .enumerate()
        .map(|(i, x)| 
            state.sample_at(&normal, (slope * x + intercept, 0.1), &format!("{}", i))
        )
        .collect::<_>()
}
pub const obs_model: DynGenFn<(f64, f64, Vec<f64>),Vec<f64>> = DynGenFn { func: _obs_model };

fn _line_model(state: &mut DynGenFnHandler<Vec<f64>,Vec<f64>>, xs: Vec<f64>) -> Vec<f64> {
    let slope = state.sample_at(&normal, (0., 1.), "slope");
    let intercept = state.sample_at(&normal, (0., 2.), "intercept");
    state.trace_at(&obs_model, (slope, intercept, xs), "ys")
}
pub const line_model: DynGenFn<Vec<f64>,Vec<f64>> = DynGenFn { func: _line_model };


// pointed model (DynGenFn version)

fn _pointed_2d_model(state: &mut DynGenFnHandler<(Bounds, DMatrix<f64>),Point>, args: (Bounds, DMatrix<f64>)) -> Point {
    let (bounds, cov) = args;
    let latent = state.sample_at(&uniform_2d, bounds, "latent");
    state.sample_at(&mvnormal, (latent, cov), "obs")
}
pub const pointed_2d_model: DynGenFn<(Bounds,DMatrix<f64>),Point> = DynGenFn { func: _pointed_2d_model };

// pointed proposal (DynGenFn version)

fn _pointed_2d_drift_proposal(state: &mut DynGenFnHandler<(Weak<Trace<(Bounds, DMatrix<f64>),DynTrie,Point>>, DMatrix<f64>),()>,
                              args: (Weak<Trace<(Bounds, DMatrix<f64>),DynTrie,Point>>, DMatrix<f64>)) -> () {
    let (trace, noise) = args;
    let trace = trace.upgrade().unwrap();
    let latent = trace.data.read::<DVector<f64>>("latent");
    state.sample_at(&mvnormal, (latent, noise), "latent");
}
pub const pointed_2d_drift_proposal: DynGenFn<(Weak<Trace<(Bounds, DMatrix<f64>),DynTrie,Point>>,DMatrix<f64>),()> = DynGenFn { func: _pointed_2d_drift_proposal };