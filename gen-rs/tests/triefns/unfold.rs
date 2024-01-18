use std::f64::consts::PI;

use gen_rs::{TrieFnState,TrieFn,Unfold,uniform,normal,mvnormal};
use nalgebra::{dvector,dmatrix};

use crate::pointed_model;
use pointed_model::types_2d::{Bounds, Point, uniform_2d};


fn polar_to_cartesian(pol: &Point) -> Point {
    dvector![pol[0]*pol[1].cos(), pol[0]*pol[1].sin()]
}

fn _spiral_kernel(g: &mut TrieFnState<(i64,Point),Point>, args: (i64,Point)) -> Point {
    let (t, prev_pol) = args;
    let pol: Point;  // polar coords
    let pos: Point;  // cartesian coords
    if t == 0 {
        let r = g.sample_at(&uniform, (0., 1.), "r");
        let theta = g.sample_at(&uniform, (0., 2.*PI), "theta");
        pol = dvector![r, theta];
        pos = polar_to_cartesian(&pol);
    } else {
        let dr = g.sample_at(&normal, (0., 0.1), "dr");
        let dtheta = g.sample_at(&normal, (0.4, 0.2), "dtheta");
        pol = dvector![prev_pol[0] + dr, prev_pol[1] + dtheta];
        pos = polar_to_cartesian(&pol);
    }
    g.sample_at(&mvnormal, (pos, dmatrix![0.001, 0.; 0., 0.001]), "obs");
    return pol;
}
pub const spiral_model: Unfold<Point> = Unfold { kernel: _spiral_kernel };