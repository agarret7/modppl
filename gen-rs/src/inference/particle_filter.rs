// mostly copied verbatim from: https://github.com/OpenGen/GenTL/blob/main/include/gentl/inference/particle_filter.h

use rand::rngs::ThreadRng;
use crate::{Trace,GenerativeFunction,ChoiceBuffer,GfDiff,dists::{self,Distribution}, mathutils::logsumexp};

pub struct ParticleSystem<X: Copy,T,U: Trace<X=(i64,X),T=T> + Clone,F: GenerativeFunction<X=(i64,X),T=T,U=U>> {
    num_particles: usize,
    model: Box<F>,
    traces: Vec<U>,
    traces_tmp: Vec<Option<U>>,

    log_weights: Vec<f64>,
    log_normalized_weights: Vec<f64>,
    two_times_log_normalized_weights: Vec<f64>,
    normalized_weights: Vec<f64>,

    parents: Vec<usize>,
    rng: ThreadRng,

    log_ml_estimate: f64
}

impl<X: Copy,T,U: Trace<X=(i64,X),T=T> + Clone,F: GenerativeFunction<X=(i64,X),T=T,U=U>> ParticleSystem<X,T,U,F> {
    fn normalize_weights(&mut self) -> f64 {
        let log_total_weight = logsumexp(&self.log_weights);
        for i in 0..self.num_particles {
            self.log_normalized_weights[i] = self.log_weights[i] - log_total_weight;
            self.two_times_log_normalized_weights[i] = 2.0 * self.log_normalized_weights[i];
            self.normalized_weights[i] = self.log_normalized_weights[i].exp();
        }
        log_total_weight
    }

    fn multinomial_resampling(&mut self) {
        for i in 0..self.num_particles {
            self.parents[i] = dists::categorical.random(&mut self.rng, self.normalized_weights.clone());
        }
    }

    pub fn new(model: F, num_particles: usize, rng: ThreadRng) -> Self {
        ParticleSystem {
            num_particles,
            model: Box::new(model),
            traces: vec![],
            traces_tmp: vec![None; num_particles],
            log_weights: vec![0.; num_particles],
            log_normalized_weights: vec![0.; num_particles],
            two_times_log_normalized_weights: vec![0.; num_particles],
            normalized_weights: vec![0.; num_particles],
            parents: vec![0; num_particles],
            rng: rng,
            log_ml_estimate: 0.
        }
    }

    pub fn init_step(
        &mut self,
        args: X,
        constraints: impl ChoiceBuffer
    ) {
        for i in 0..self.num_particles {
            let trace = self.model.generate(&mut self.rng, (1, args), constraints.clone());
            self.traces.push(trace);
            self.log_weights[i] = self.traces[i].get_score();
        }
    }

    pub fn step(&mut self, diff: GfDiff, constraints: impl ChoiceBuffer) {
        for i in 0..self.traces.len() {
            let args = *self.traces[i].get_args();
            let new_args = (args.0 + 1, args.1);
            let old_score = self.traces[i].get_score();
            self.model.update(&mut self.rng, &mut self.traces[i], new_args, diff.clone(), constraints.clone());
            self.log_weights[i] += self.traces[i].get_score() - old_score;
        }
    }

    pub fn effective_sample_size(&self) -> f64 {
        (-logsumexp(&self.two_times_log_normalized_weights)).exp()
    }

    pub fn resample(&mut self) -> f64 {
        let log_total_weight = self.normalize_weights();
        self.log_ml_estimate += log_total_weight - (self.num_particles as f64).ln();

        self.multinomial_resampling();

        for i in 0..self.num_particles {
            self.traces_tmp[i] = Some(self.traces[self.parents[i]].clone());
        }
        for i in 0..self.num_particles {
            self.traces[i] = self.traces_tmp[i].take().unwrap();
        }
        self.log_weights.fill(0.);
        log_total_weight
    }

    pub fn log_marginal_likelihood_estimate(&self) -> f64 {
        self.log_ml_estimate + logsumexp(&self.log_weights) - (self.num_particles as f64).ln()
    }
}