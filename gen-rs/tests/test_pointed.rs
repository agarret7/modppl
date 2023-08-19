mod pointed_model;
use pointed_model::types_2d;
use rand::rngs::ThreadRng;
use nalgebra::dvector;

use gen_rs::Distribution;


#[test]
fn test_uniform2d() {
    let mut rng = ThreadRng::default();
    let bounds = types_2d::Bounds { xmin: 0., xmax: 2.5, ymin: -1., ymax: 0.25 };

    let samples = (0..50000).map(|_| types_2d::uniform_2d.random(&mut rng, bounds)).collect::<Vec<types_2d::Point>>();
    for sample in samples {
        assert!(sample[0] >= bounds.xmin);
        assert!(sample[0] <= bounds.xmax);
        assert!(sample[1] >= bounds.ymin);
        assert!(sample[1] <= bounds.ymax);
        approx::assert_abs_diff_eq!(types_2d::uniform_2d.logpdf(&sample, bounds), -1.1394342831883648, epsilon=f64::EPSILON);
    }

    assert_eq!(types_2d::uniform_2d.logpdf(&dvector![-1., 0.], bounds), -f64::INFINITY);
}