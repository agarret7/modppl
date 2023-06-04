use rand::rngs::ThreadRng;
use crate::{dists, types_2d};
use dists::Distribution;

pub struct ParticleFilterState {
    pub traces: Vec<Vec<types_2d::Point>>,
    pub weights: Vec<f32>
}


fn logsumexp(xs: &Vec<f32>) -> f32 {
    let max = xs.iter().cloned().fold(-1./0. /* -inf */, f32::max);
    const negative_infinity: f32 = -f32::INFINITY;
    if max == -negative_infinity {
        return -negative_infinity
    } else {
        let mut sum_exp = 0.;
        for x in xs {
            sum_exp += (x - max).exp();
        }
        return sum_exp.ln();
    }
}

impl ParticleFilterState {

    pub fn new(rng: &mut ThreadRng, num_samples: u32, b: &types_2d::Bounds, obs: &types_2d::Point) -> ParticleFilterState {
        let mut points = vec![];
        let mut weights = vec![];
        let obs_std = 0.1;
        for _ in 0..num_samples {
            let point = dists::uniform_2d.random(rng, b);
            let weight = dists::uniform_2d.logpdf(&point, b)
                + dists::normal.logpdf(&point.x, &(obs.x, obs_std))
                + dists::normal.logpdf(&point.y, &(obs.y, obs_std));
            points.push(point);
            weights.push(weight);
        }
        ParticleFilterState { traces: points.into_iter().map(|p| vec![p]).collect::<Vec<Vec<types_2d::Point>>>(), weights: weights }
    }

    pub fn step(
        &mut self,
        rng: &mut ThreadRng,
        new_t: usize, 
        new_obs: &types_2d::Point
    ) {
        for i in 0..self.weights.len() {
            let old_points = &self.traces[new_t-1];
            let old_weights = self.weights[i];
            let x_std = 10.;
            let y_std = 10.;
            let dx = dists::normal.random(rng, &(0.,x_std));
            let dy = dists::normal.random(rng, &(0.,y_std));
            let proposed_p = types_2d::Point { x: old_points[0].x + dx, y: old_points[0].y + dy };
            let inc = dists::normal.logpdf(&(new_obs.x as f32), &(proposed_p.x as f32, x_std))
                    + dists::normal.logpdf(&(new_obs.y as f32), &(proposed_p.y as f32, y_std));
            // *pf_state.points[new_t] = 
        }
    }

    fn maybe_resample(&mut self, ess_threshold: f64) -> bool {
        false
    }

    pub fn sample_unweighted_traces(&mut self, rng: &mut ThreadRng, num_samples: u32) {
        let norm_f = logsumexp(&self.weights);
        let probs = &self.weights.iter()
            .map(|w| (w - norm_f).exp())
            .collect::<Vec<f32>>();
        self.traces = (0..self.traces.len())
            .map(|_| dists::categorical.random(rng, probs))
            .map(|idx| self.traces[idx].clone())
            .collect::<Vec<Vec<types_2d::Point>>>();
        self.weights = self.traces.iter().map(|_| 0.).collect::<Vec<f32>>();
    }

}