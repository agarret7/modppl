use rand::rngs::ThreadRng;
use modppl::{Distribution,u01};
use nalgebra::{DVector,dvector};


#[derive(Clone, Copy)]
pub struct Bounds { pub xmin: f64, pub xmax: f64, pub ymin: f64, pub ymax: f64 }
pub type Point = DVector<f64>;


pub struct Uniform2D { }
pub const uniform_2d: Uniform2D = Uniform2D { };

impl Distribution<Point,Bounds> for Uniform2D {
    fn logpdf(&self, p: &Point, b: Bounds) -> f64 {
        return if b.xmin <= p[0] && p[0] <= b.xmax && b.ymin <= p[1] && p[1] <= b.ymax {
            -((b.xmax - b.xmin) as f64 * (b.ymax - b.ymin) as f64).ln()
        } else {
            f64::NEG_INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, b: Bounds) -> Point {
        assert!(b.xmax > b.xmin);
        assert!(b.ymax > b.ymin);
        dvector![
            u01(rng)*(b.xmax - b.xmin) + b.xmin,
            u01(rng)*(b.ymax - b.ymin) + b.ymin
        ]
    }
}
