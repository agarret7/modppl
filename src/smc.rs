use std::rc::Rc;
use rand::rngs::ThreadRng;
use crate::{
    dists::{self, Distribution},
    types_2d,
    mathutils::logsumexp,
    vec_trace::VecTrace
};

pub struct ParticleFamily {
    pub traces: Vec<VecTrace>,
}


impl ParticleFamily {

    pub fn new(rng: &mut ThreadRng, num_samples: u32, bounds: types_2d::Bounds, obs: Rc<types_2d::Point>) -> ParticleFamily {
        ParticleFamily {
            traces: (0..num_samples)
                .map(|_| VecTrace::generate(rng, bounds, Rc::clone(&obs)))
                .collect::<Vec<VecTrace>>()
        }
    }

    pub fn get_weights(&self) -> Vec<f32> {
        self.traces.iter().map(|t| t.get_score()).collect::<Vec<f32>>()
    }

    pub fn nourish(
        &mut self,
        rng: &mut ThreadRng,
        obs: Rc<types_2d::Point>
    ) {
        for t in self.traces.iter_mut() {
            t.grow(rng, Rc::clone(&obs));
        }
    }

    fn maybe_resample(&mut self, ess_threshold: f64) -> bool {
        false
    }

    pub fn sample_unweighted_traces(&mut self, rng: &mut ThreadRng, num_samples: u32) {
        let norm_f = logsumexp(&self.get_weights().iter().map(|x| *x).collect::<Vec<f32>>());
        let probs = &self.get_weights().iter()
            .map(|w| (w - norm_f).exp())
            .collect::<Vec<f32>>();
        self.traces = (0..num_samples)
            .map(|_| dists::categorical.random(rng, probs))
            .map(|idx| {
                let t = &self.traces[idx];
                let (points, observations) = t.get_choices();
                VecTrace::new(
                    t.get_args().clone(),
                    points.clone(),
                    observations.clone(),
                    0.
                )
            })
            .collect::<Vec<VecTrace>>();
    }

}