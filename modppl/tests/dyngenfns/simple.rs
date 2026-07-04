use modppl::prelude::*;
use nalgebra::{DMatrix, DVector};
use std::sync::Weak;

use crate::pointed_model;
use pointed_model::types_2d::{uniform_2d, Bounds, Point};

// bayesian linear regression model
dyngen!(
    fn obs_model(slope: Real, intercept: Real, xs: Vec<Real>) -> Vec<Real> {
        xs.into_iter()
            .enumerate()
            .map(|(i, x)| normal(slope * x + intercept, 0.1) %= &format!("{}", i))
            .collect::<_>()
    }
);

dyngen!(
    pub fn line_model(xs: Vec<Real>) -> Vec<Real> {
        let slope = normal(0., 1.) %= "slope";
        let intercept = normal(0., 2.) %= "intercept";
        obs_model(slope, intercept, xs) /= "ys"
    }
);

// pointed model (DynGenFn version)
dyngen!(
    pub fn pointed_2d_model(bounds: Bounds, cov: DMatrix<Real>) -> Point {
        let latent = uniform_2d(bounds) %= "latent";
        mvnormal(latent, cov) %= "obs"
    }
);

// pointed proposal (DynGenFn version)
dyngen!(
    pub fn pointed_2d_drift_proposal(
        trace: Weak<DynTrace<(Bounds, DMatrix<Real>), Point>>,
        noise: DMatrix<Real>,
    ) -> () {
        let trace = trace.upgrade().unwrap();
        let prev_latent = trace.data.read::<DVector<Real>>("latent");
        mvnormal(prev_latent, noise) %= "latent";
    }
);
