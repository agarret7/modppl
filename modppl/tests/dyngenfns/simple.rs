use std::sync::Weak;
use modppl::prelude::*;
use nalgebra::{DMatrix, DVector};

use crate::pointed_model;
use pointed_model::types_2d::{Bounds, Point, uniform_2d};


// bayesian linear regression model
dyngen!(
fn obs_model(slope: f64, intercept: f64, xs: Vec<f64>) -> Vec<f64> {
    xs.into_iter()
        .enumerate()
        .map(|(i, x)| normal(slope * x + intercept, 0.1) %= &format!("{}", i))
        .collect::<_>()
});

dyngen!(
pub fn line_model(xs: Vec<f64>) -> Vec<f64> {
    let slope = normal(0., 1.) %= "slope";
    let intercept = normal(0., 2.) %= "intercept";
    obs_model(slope, intercept, xs) /= "ys"
});


// pointed model (DynGenFn version)
dyngen!(
pub fn pointed_2d_model(bounds: Bounds, cov: DMatrix<f64>) -> Point {
    let latent = uniform_2d(bounds) %= "latent";
    mvnormal(latent, cov) %= "obs"
});

// pointed proposal (DynGenFn version)
dyngen!(
pub fn pointed_2d_drift_proposal(trace: Weak<DynTrace<(Bounds,DMatrix<f64>),Point>>, noise: DMatrix<f64>) -> () {
    let trace = trace.upgrade().unwrap();
    let prev_latent = trace.data.read::<DVector<f64>>("latent");
    mvnormal(prev_latent, noise) %= "latent";
});