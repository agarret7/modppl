use std::f64::consts::PI;

use modppl::prelude::*;
use nalgebra::{dvector,dmatrix};

use crate::pointed_model;
use pointed_model::types_2d::Point;


fn polar_to_cartesian(pol: &Point) -> Point {
    dvector![pol[0]*pol[1].cos(), pol[0]*pol[1].sin()]
}

dyngen!(
fn spiral_kernel(t: i64, prev_pol: Point) -> Point {
    let pol: Point;  // polar coords
    let pos: Point;  // cartesian coords
    if t == 0 {
        let r = uniform(0., 1.) %= "r";
        let theta = uniform(0., 2.*PI) %= "theta";
        pol = dvector![r, theta];
        pos = polar_to_cartesian(&pol);
    } else {
        let dr = normal(0., 0.1) %= "dr";
        let dtheta = normal(0.4, 0.2) %= "dtheta";
        pol = dvector![prev_pol[0] + dr, prev_pol[1] + dtheta];
        pos = polar_to_cartesian(&pol);
    }
    mvnormal(pos, dmatrix![0.001, 0.; 0., 0.001]) %= "obs";
    return pol;
});

pub const spiral_model: DynUnfold<Point> = DynUnfold { kernel: spiral_kernel };