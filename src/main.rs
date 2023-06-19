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

fn main() {

}


// fn main() -> std::io::Result<()> {
//     let timesteps = 100;
//     let num_samples = 5000;
//     let bounds = types_2d::Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };

//     create_dir_all("data")?;

//     // in this example our ground-truth observations are fully synthetic.
//     // to make it interesting, we sample a random initial orientation.
//     let observations = simulate_loop(&bounds, timesteps);

//     // first we plot the observation over time (prototype) and run "mental" inference
//     let data = serde_json::to_string(&observations).unwrap();
//     write("data/observations.json", data)?;

//     // then we connect to a thread of randomness
//     let mut rng = ThreadRng::default();

//     let mut pf_state = smc::ParticleFamily::new(&mut rng, num_samples, bounds, Rc::clone(&observations[0]));

//     // then we visualize how our initial particles compare to the initial observation
//     let data = pf_state.traces.iter().map(|t| t.get_choices().0).collect::<Vec<&Vec<types_2d::Point>>>();
//     let json = serde_json::to_string(&data)?;
//     write("data/initial_traces.json", json)?;

//     // we copy the most promising particles according to importance weights
//     // that account for a "normal" likelihood.
//     let num_resamples = 1;
//     pf_state.sample_unweighted_traces(&mut rng, num_resamples);

//     // then we visualize how our updated guesses characterize the initial uncertainty
//     let data = pf_state.traces.iter().map(|t| t.get_choices().0).collect::<Vec<&Vec<types_2d::Point>>>();
//     let json = serde_json::to_string(&data)?;
//     write("data/resampled_initial_traces.json", json)?;

//     // drift metropolis-hastings rejuvenation demo
//     for (t, obs) in observations[1..10].iter().enumerate() {
//         println!("T = {}", t+1);
//         pf_state.nourish(&mut rng, Rc::clone(obs));
//         for iter in 0..100 {
//             println!("  iter = {}", iter);
//             print!("    |");
//             for (i, tr) in pf_state.traces.iter_mut().enumerate() {
//                 match inference::drift_metropolis_hastings_rejuv(&mut rng, &tr) {
//                     Some(new_tr) => { print!("{}:ACC|", i); *tr = new_tr }
//                     None => { print!("{}:REJ|", i); }
//                 }
//                 dbg!(tr.get_score());
//             }
//             // save each iteration of MH for visualization
//             let data = pf_state.traces.iter().map(|t| t.get_choices().0).collect::<Vec<&Vec<types_2d::Point>>>();
//             let json = serde_json::to_string(&data)?;
//             write(format!("data/mh_t_{}_iter_{}.json", t+1, iter), json)?;
//             println!("");
//         }
//     }

//     // Metropolis-adjusted Langevin Ascent demo
//     // ...

//     // Hamiltonian Monte Carlo demo
//     // ...

//     Ok(())
// }