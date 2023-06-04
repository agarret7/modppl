use std::f32::consts::PI;
use std::fs::{File, write, create_dir_all};
use math::sample;
use rand::rngs::ThreadRng;
use serde_json;

mod dists;
mod types_2d;
mod smc;

use dists::Distribution;
use smc::ParticleFilterState;


// Functions

fn simulate_loop(b: &types_2d::Bounds, T: i32) -> Vec<types_2d::Point> {
    let xrange = (b.xmax - b.xmin) as f32;
    let yrange = (b.ymax - b.ymin) as f32;
    let cx = xrange / 2. + b.xmin;
    let cy = yrange / 2. + b.ymin;
    let r = f32::max(b.xmax - b.xmin, b.ymax - b.ymin) / 3.;
    let mut observations = vec![];
    for t in 1..T {
        let u = 20.*PI*t as f32 / T as f32;
        let t = 2.*PI*t as f32 / T as f32;
        let obs = types_2d::Point { x: r*t.cos() + r/8.*u.sin(), y: r*t.sin() + r/8.*u.cos() };
        observations.push(obs);
    }
    observations
}


fn main() -> std::io::Result<()> {
    let T = 100;
    let num_samples = 10000;
    let b = types_2d::Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };

    create_dir_all("data")?;

    // in this example our ground-truth observations are fully synthetic.
    // to make it interesting, we sample a random initial orientation.
    // We also assume our particles undergo Brownian (Gaussian drift)
    // motion by default. Later we'll consider "smarter" update strategies
    // that use enumerative strategies to improve the marginal likelihood.
    let observations = simulate_loop(&b, T);

    // first we plot the observation over time (prototype) and run "mental" inference
    let data = serde_json::to_string(&observations).unwrap();
    write("data/observations.json", data)?;

    // then we connect to a thread of randomness
    let mut rng = ThreadRng::default();

    // we initialize blind guesses of where the true object might be, represented as particle "guesses"
    let mut pf_state = ParticleFilterState::new(&mut rng, num_samples, &b, &observations[0]);

    // then we visualize how our initial particles compare to the initial observation
    let data = serde_json::to_string(&pf_state.traces)?;
    write("data/initial_particles.json", data)?;

    // we copy the most promising particles according to importance weights
    // that account for a "normal" likelihood. Note the normal distribution is a
    // surprisingly robust and general-purpose prior, thanks to central limit theorem
    pf_state.sample_unweighted_traces(&mut rng, num_samples);

    // todo: then we visualize how our updated guesses characterize the initial uncertainty
    // resampled_initial_particles.json
    let data = serde_json::to_string(&pf_state.traces)?;
    write("data/resampled_initial_particles.json", data)?;

    // for (t, obs) in observations.iter().enumerate() {
    //     pf_state.step(&mut rng, t+1,&obs);
    //     // todo: save all proposed particle extensions
    //     // todo: save the accepted extensions and historical traces
    // }

    // todo: save all proposals (dynamic_proposals.json)
    // todo: save the final traces (final_traces.json)
    Ok(())
}