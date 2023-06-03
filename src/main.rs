use std::f32::consts::PI;
use rand::rngs::ThreadRng;
use serde_json;

pub mod types_2d; pub mod dists;
use dists::Distribution;


struct ParticleFilterState {
    points: Vec<Vec<types_2d::Point>>,
    weights: Vec<f32>
}

impl ParticleFilterState {

    fn new(rng: &mut ThreadRng, num_samples: u32, b: &types_2d::Bounds) -> ParticleFilterState {
        let mut points = vec![];
        let mut weights = vec![];
        for _ in 0..num_samples {
            let point = dists::uniform_2d.random(rng, b);
            let weight = dists::uniform_2d.logpdf(b, &point);
            points.push(point);
            weights.push(weight);
        }
        ParticleFilterState { points: vec![points], weights: weights }
    }

    fn step(
        &mut self,
        rng: &mut ThreadRng,
        new_t: usize, 
        new_obs: &types_2d::Point
    ) {
        for i in 1..self.weights.len() {
            let old_points = &self.points[new_t-1];
            let old_weights = self.weights[i];
            let x_std = 10.;
            let y_std = 10.;
            let dx = dists::normal.random(rng, &(0.,x_std));
            let dy = dists::normal.random(rng, &(0.,y_std));
            let proposed_p = types_2d::Point { x: old_points[0].x + dx, y: old_points[0].y + dy };
            let inc = dists::normal.logpdf(&(proposed_p.x as f32, x_std), &(new_obs.x as f32))
                    + dists::normal.logpdf(&(proposed_p.y as f32, y_std), &(new_obs.y as f32));
            // *pf_state.points[new_t] = 
        }
    }

    fn maybe_resample(&mut self, ess_threshold: f64) -> bool {
        false
    }

    fn sampled_unweighted_points(&mut self, rng: &mut ThreadRng, num_samples: u32) {
    }

}
 

// Functions

fn simulate_loop(b: &types_2d::Bounds, T: i32) -> Vec<types_2d::Point> {
    let xrange = (b.xmax - b.xmin) as f32;
    let yrange = (b.ymax - b.ymin) as f32;
    let cx = xrange / 2. + b.xmin;
    let cy = yrange / 2. + b.ymin;
    let r = f32::max(b.xmax - b.xmin, b.ymax - b.ymin) / 10.;
    dbg!(r);
    let mut observations = vec![];
    for t in 1..T {
        let u = (t as f32) / (10.*PI);
        let t = (t as f32) / (2.*PI);
        dbg!(u);
        dbg!(t);
        let obs = types_2d::Point { x: r*t.cos(), y: r*t.sin() };
        observations.push(obs);
    }
    observations
}

fn importance_resample(particles: &Vec<(types_2d::Point,f32)>, n: u32, obs: &types_2d::Point) -> Vec<(types_2d::Point,f32)> { 
    let new_particles = vec![];
    // generate random indices proportional to categorical
    for p in particles {
    }
    new_particles
}

// fn do_mcmc_rejuvenation(particles, proposal, observations) {

// }

// fn sampled_unweighted_traces(state, proposal, observations) {

// }


fn main() {
    let T = 30;
    let num_samples = 1;
    let b = types_2d::Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };

    // in this example our ground-truth observations are fully synthetic.
    // to make it interesting, we sample a random initial orientation.
    // We also assume our particles undergo Brownian (Gaussian drift)
    // motion by default. Later we'll consider "smarter" update strategies
    // that use enumerative strategies to improve the marginal likelihood.
    let observations = simulate_loop(&b, T);

    let serialized = serde_json::to_string(&observations).unwrap();
    println!("serialized = {}", serialized);

    // first we plot the observation over time (prototype) and run "mental" inference
    // todo: save and review using matplotlib (observations.json)

    // then we connect to a thread of randomness
    let mut rng = ThreadRng::default();

    // we initialize blind guesses of where the true object might be, represented as particle "guesses"
    let mut pf_state = ParticleFilterState::new(&mut rng, num_samples, &b);

    // todo: then we visualize how our initial particles compare to the initial observation
    // initial_particles.json

    // todo: we copy the most promising particles according to importance weights
    // that account for a "normal" likelihood. Note the normal distribution is a
    // surprisingly robust and general-purpose prior, thanks to central limit theorem

    // todo: then we visualize how our updated guess characterizes the initial uncertainty
    // resampled_initial_particles.json

    for (t, obs) in observations.iter().enumerate() {
        pf_state.step(&mut rng, t+1,&obs);
        // todo: save all proposed particle extensions
        // todo: save the accepted extensions and historical traces
    }

    // todo: save all proposals (dynamic_proposals.json)
    // todo: save the final traces (final_traces.json)
}