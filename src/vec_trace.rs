// For experimentation a "trace" is just a vector of 2D points

// Currently the only model supported is in smc.rs and is:
// - prior: Uniform2D
// - kernel: Normal X Normal
// - likelihood: Normal X Normal

use std::rc::Rc;
use rand::rngs::ThreadRng;
use crate::{
    dists::{self, Distribution},
    types_2d
};

#[derive(Clone)]
pub struct VecTrace {
    // all fields are private,
    // "hidden" behind an interface
    args: types_2d::Bounds,
    latent_choices: Vec<types_2d::Point>,
    obs_choices: Vec<Rc<types_2d::Point>>,
    score: f32
}

impl VecTrace {
    pub fn new(
        args: types_2d::Bounds,
        latent_choices: Vec<types_2d::Point>,
        obs_choices: Vec<Rc<types_2d::Point>>,
        score: f32
    ) -> VecTrace {
        VecTrace {
            args: args,
            latent_choices: latent_choices,
            obs_choices: obs_choices,
            score: score
        }
    }

    pub fn generate(rng: &mut ThreadRng, bounds: types_2d::Bounds, obs: Rc<types_2d::Point>) -> VecTrace {
        let obs_std = 0.02;  // todo: make this a parameter as well
        let point = dists::uniform_2d.random(rng, &bounds);
        let weight = dists::uniform_2d.logpdf(&point, &bounds)                // prior
                    + dists::normal.logpdf(&obs.x, &(point.x, obs_std))       // likelihood
                    + dists::normal.logpdf(&obs.y, &(point.y, obs_std));
        VecTrace {
            args: bounds,
            latent_choices: vec![point],
            obs_choices: vec![obs],
            score: weight
        }
    }

    // feed in an observation and extend the trace
    pub fn grow(&mut self, rng: &mut ThreadRng, obs: Rc<types_2d::Point>) -> f32 {
        let obs_std = 0.02;
        let x_std = 0.01;
        let y_std = 0.01;
        let dx = dists::normal.random(rng, &(0.,x_std));
        let dy = dists::normal.random(rng, &(0.,y_std));
        let last_p = self.latent_choices.last().unwrap();
        let proposed_p = types_2d::Point {
            x: last_p.x + dx,
            y: last_p.y + dy
        };
        self.score += dists::normal.logpdf(&proposed_p.x, &(last_p.x, x_std))
            + dists::normal.logpdf(&proposed_p.y, &(last_p.y, y_std))
            + dists::normal.logpdf(&obs.x, &(proposed_p.x, obs_std))
            + dists::normal.logpdf(&obs.y, &(proposed_p.y, obs_std));
        self.latent_choices.push(proposed_p);
        self.score
    }

    // pick a point in history and revise our latents
    // return the self.weight - fwd_weight + bwd_weight
    // pub fn update(&self, t: usize, point: types_2d::Point) -> (VecTrace, f32) {
    //     (self, self.score)
    // }

    pub fn get_args(&self) -> &types_2d::Bounds {
        &self.args
    }

    pub fn get_choices(&self) -> (&Vec<types_2d::Point>, &Vec<Rc<types_2d::Point>>) {
        (&self.latent_choices, &self.obs_choices)
    }

    pub fn get_retval(&self) -> &Vec<Rc<types_2d::Point>> {
        &self.obs_choices
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

}