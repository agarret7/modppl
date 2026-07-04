use modppl::{u01, Distribution, Real};
use nalgebra::{dvector, DVector};
use rand::rngs::ThreadRng;

#[derive(Clone, Copy)]
pub struct Bounds {
    pub xmin: Real,
    pub xmax: Real,
    pub ymin: Real,
    pub ymax: Real,
}
pub type Point = DVector<Real>;

pub struct Uniform2D {}
pub const uniform_2d: Uniform2D = Uniform2D {};

impl Distribution<Point, Bounds> for Uniform2D {
    fn logpdf(&self, p: &Point, b: Bounds) -> Real {
        return if b.xmin <= p[0] && p[0] <= b.xmax && b.ymin <= p[1] && p[1] <= b.ymax {
            -((b.xmax - b.xmin) as Real * (b.ymax - b.ymin) as Real).ln()
        } else {
            Real::NEG_INFINITY
        };
    }

    fn random(&self, rng: &mut ThreadRng, b: Bounds) -> Point {
        assert!(b.xmax > b.xmin);
        assert!(b.ymax > b.ymin);
        dvector![
            u01(rng) * (b.xmax - b.xmin) + b.xmin,
            u01(rng) * (b.ymax - b.ymin) + b.ymin
        ]
    }
}
