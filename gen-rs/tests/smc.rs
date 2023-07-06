use std::{
    rc::Rc,
    f64::consts::PI,
    // fs::{write, create_dir_all}
};
use gen_rs::modeling::dists;
use nalgebra::dvector;
use rand::rngs::ThreadRng;
// use serde_json;

pub mod pointed;
use pointed::types_2d::{Bounds,Point};


// difficult filtering problem
fn simulate_loop(rng: &mut ThreadRng, bounds: &Bounds, timesteps: i64) -> Vec<Rc<Point>> {
    let init_angle = dists::u01(rng) * 2.*PI;

    let xrange = (bounds.xmax - bounds.xmin) as f64;
    let yrange = (bounds.ymax - bounds.ymin) as f64;
    let center = dvector![
        xrange / 2. + bounds.xmin,
        yrange / 2. + bounds.ymin
    ];
    let radius = f64::max(bounds.xmax - bounds.xmin, bounds.ymax - bounds.ymin) / 3.;
    let mut observations = vec![];
    for t in 0..timesteps {
        let u = 20.*PI*(t as f64 + init_angle) as f64 / timesteps as f64;
        let t = 2.*PI*(t as f64 + init_angle) as f64 / timesteps as f64;
        let obs = dvector![
            center[0] + radius*t.cos() + radius/8.*u.sin(),
            center[1] + radius*t.sin() + radius/8.*u.cos()
        ];
        observations.push(Rc::new(obs));
    }
    observations
}

#[ignore]
#[test]
fn test_smc() {
    let mut rng = ThreadRng::default();
    const NUM_TIMESTEPS: i64 = 100;

    let bounds = Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1.};
    let data = simulate_loop(&mut rng, &bounds, NUM_TIMESTEPS);

    dbg!(data);
    // todo: implement me!

    assert!(false);
}