use std::{
    rc::Rc,
    f32::consts::PI,
    fs::{write, create_dir_all}
};
use rand::rngs::ThreadRng;
use serde_json;

use genark::types_2d;
// use genark::inference;


fn simulate_loop(bounds: &types_2d::Bounds, timesteps: i32) -> Vec<Rc<types_2d::Point>> {
    let xrange = (bounds.xmax - bounds.xmin) as f32;
    let yrange = (bounds.ymax - bounds.ymin) as f32;
    let center = types_2d::Point {
        x: xrange / 2. + bounds.xmin,
        y: yrange / 2. + bounds.ymin
    };
    let radius = f32::max(bounds.xmax - bounds.xmin, bounds.ymax - bounds.ymin) / 3.;
    let mut observations = vec![];
    for t in 0..timesteps {
        let u = 20.*PI*t as f32 / timesteps as f32;
        let t = 2.*PI*t as f32 / timesteps as f32;
        let obs = types_2d::Point {
            x: center.x + radius*t.cos() + radius/8.*u.sin(),
            y: center.y + radius*t.sin() + radius/8.*u.cos()
        };
        observations.push(Rc::new(obs));
    }
    observations
}
