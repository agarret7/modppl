use rand::rngs::ThreadRng;
use gen_rs::dists::{self, Distribution};
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
            -f64::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, b: Bounds) -> Point {
        assert!(b.xmax > b.xmin);
        assert!(b.ymax > b.ymin);
        dvector![
            dists::u01(rng)*(b.xmax - b.xmin) + b.xmin,
            dists::u01(rng)*(b.ymax - b.ymin) + b.ymin
        ]
    }
}

#[test]
fn test_uniform2d() {
    let mut rng = ThreadRng::default();
    let bounds = Bounds { xmin: 0., xmax: 2.5, ymin: -1., ymax: 0.25 };

    let samples = (0..50000).map(|_| uniform_2d.random(&mut rng, bounds)).collect::<Vec<Point>>();
    for sample in samples {
        assert!(sample[0] >= bounds.xmin);
        assert!(sample[0] <= bounds.xmax);
        assert!(sample[1] >= bounds.ymin);
        assert!(sample[1] <= bounds.ymax);
        approx::assert_abs_diff_eq!(uniform_2d.logpdf(&sample, bounds), -1.1394342831883648, epsilon=f64::EPSILON);
    }

    assert_eq!(uniform_2d.logpdf(&dvector![-1., 0.], bounds), -f64::INFINITY);
}