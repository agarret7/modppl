use std::rc::Rc;
use std::f32::consts::PI;
use std::fs::{write, create_dir_all};
use rand::rngs::ThreadRng;
use serde_json;

use gen_reflex::types_2d;
use gen_reflex::smc;

use smc::ParticleFamily;


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


fn main() -> std::io::Result<()> {
    let timesteps = 100;
    let num_samples = 5000;
    let bounds = types_2d::Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };

    create_dir_all("data")?;

    // in this example our ground-truth observations are fully synthetic.
    // to make it interesting, we sample a random initial orientation.
    // We also assume our particles undergo Brownian (Gaussian drift)
    // motion by default. Later we'll consider "smarter" update strategies
    // that use enumerative strategies to improve the marginal likelihood.
    let observations = simulate_loop(&bounds, timesteps);

    // first we plot the observation over time (prototype) and run "mental" inference
    let data = serde_json::to_string(&observations).unwrap();
    write("data/observations.json", data)?;

    // then we connect to a thread of randomness
    let mut rng = ThreadRng::default();

    // we initialize blind guesses of where the true object might be, represented as particle "guesses"
    let mut pf_state = ParticleFamily::new(&mut rng, num_samples, bounds, Rc::clone(&observations[0]));

    // then we visualize how our initial particles compare to the initial observation
    let data = pf_state.traces.iter().map(|t| t.get_choices().0).collect::<Vec<&Vec<types_2d::Point>>>();
    let json = serde_json::to_string(&data)?;
    write("data/initial_particles.json", json)?;

    // we copy the most promising particles according to importance weights
    // that account for a "normal" likelihood. 
    pf_state.sample_unweighted_traces(&mut rng, num_samples);

    // then we visualize how our updated guesses characterize the initial uncertainty
    let data = pf_state.traces.iter().map(|t| t.get_choices().0).collect::<Vec<&Vec<types_2d::Point>>>();
    let json = serde_json::to_string(&data)?;
    write("data/resampled_initial_particles.json", json)?;

    // for (t, obs) in observations.iter().enumerate() {
    //     pf_state.step(&mut rng, t+1,&obs);
    //     todo: perform one rejuvenation step of mh
    //     todo: save all proposed particle extensions (regardless of acceptance)
    //     todo: save the actual extensions and historical traces
    // }

    // todo: save all proposals (dynamic_proposals.json)
    // todo: save the final traces (final_traces.json)

    Ok(())
}