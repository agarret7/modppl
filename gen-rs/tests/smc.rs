// use std::{
//     rc::Rc,
//     f64::consts::PI,
//     fs::{write, create_dir_all}
// };
// use rand::rngs::ThreadRng;
// use serde_json;

// pub mod pointed;
// use pointed::types_2d;


// fn simulate_loop(bounds: &types_2d::Bounds, timesteps: i32) -> Vec<Rc<types_2d::Point>> {
//     let xrange = (bounds.xmax - bounds.xmin) as f64;
//     let yrange = (bounds.ymax - bounds.ymin) as f64;
//     let center = types_2d::Point {
//         x: xrange / 2. + bounds.xmin,
//         y: yrange / 2. + bounds.ymin
//     };
//     let radius = f64::max(bounds.xmax - bounds.xmin, bounds.ymax - bounds.ymin) / 3.;
//     let mut observations = vec![];
//     for t in 0..timesteps {
//         let u = 20.*PI*t as f64 / timesteps as f64;
//         let t = 2.*PI*t as f64 / timesteps as f64;
//         let obs = types_2d::Point {
//             x: center.x + radius*t.cos() + radius/8.*u.sin(),
//             y: center.y + radius*t.sin() + radius/8.*u.cos()
//         };
//         observations.push(Rc::new(obs));
//     }
//     observations
// }
