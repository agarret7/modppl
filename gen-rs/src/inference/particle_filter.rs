// mostly copied verbatim from: https://github.com/OpenGen/GenTL/blob/main/include/gentl/inference/particle_filter.h

use rand::rngs::ThreadRng;
use crate::{Trace,GenFn,GfDiff,Distribution,categorical,mathutils::logsumexp};


/// Basic particle filter for generative functions with a time parameter as the first input argument.
pub struct ParticleSystem<Args: Clone,Data: Clone,Ret: Clone,F: GenFn<(i64,Args),Data,Ret>> {
    num_particles: usize,
    model: Box<F>,

    /// Persistent traces contained within the system
    pub traces: Vec<Trace<(i64,Args),Data,Ret>>,

    log_weights: Vec<f64>,
    log_normalized_weights: Vec<f64>,
    two_times_log_normalized_weights: Vec<f64>,
    normalized_weights: Vec<f64>,

    parents: Vec<usize>,
    rng: ThreadRng,

    log_ml_estimate: f64
}

impl<Args: Clone,Data: Clone,Ret: Clone,F: GenFn<(i64,Args),Data,Ret>> ParticleSystem<Args,Data,Ret,F> {
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
            self.parents[i] = categorical.random(&mut self.rng, self.normalized_weights.clone()) as usize;
        }
    }

    /// Construct a new particle filter under the `model` with `num_particles` particles.
    pub fn new(model: F, num_particles: usize, rng: ThreadRng) -> Self {
        ParticleSystem {
            num_particles,
            model: Box::new(model),
            traces: vec![],
            log_weights: vec![0.; num_particles],
            log_normalized_weights: vec![0.; num_particles],
            two_times_log_normalized_weights: vec![0.; num_particles],
            normalized_weights: vec![0.; num_particles],
            parents: vec![0; num_particles],
            rng: rng,
            log_ml_estimate: 0.
        }
    }

    /// Initialize the particle filter by generating `self.num_particles` traces from the `model` with `(1, args)`.
    pub fn init_step(
        &mut self,
        args: Args,
        constraints: Data
    ) {
        for i in 0..self.num_particles {
            let (trace, log_weight) = self.model.generate((1, args.clone()), constraints.clone());
            self.traces.push(trace);
            self.log_weights[i] = log_weight;
        }
    }

    /// Extend the current filter from `t` to `t+1` with new `constraints`.
    pub fn step(self, constraints: Data) -> Self {
        let mut tmp_traces = vec![];
        let mut tmp_log_weights = vec![];
        for (i, trace) in self.traces.into_iter().enumerate() {
            let args = trace.args.clone();
            let new_args = (args.0 + 1, args.1);
            let (new_trace, _, log_weight) = self.model.update(trace, new_args, GfDiff::Extend, constraints.clone());
            tmp_traces.push(new_trace);
            tmp_log_weights.push(self.log_weights[i] + log_weight);
        }
        ParticleSystem {
            num_particles: self.num_particles,
            model: self.model,
            traces: tmp_traces,
            log_weights: tmp_log_weights,
            log_normalized_weights: self.log_normalized_weights,
            two_times_log_normalized_weights: self.two_times_log_normalized_weights,
            normalized_weights: self.normalized_weights,
            parents: self.parents,
            rng: self.rng,
            log_ml_estimate: self.log_ml_estimate
        }
    }

    /// Calculate the effective sample size (ESS) with the current paticle weights.
    pub fn effective_sample_size(&self) -> f64 {
        (-logsumexp(&self.two_times_log_normalized_weights)).exp()
    }

    /// Perform multinomial resampling based on the normalized particle weights, and return the log total weight.
    pub fn resample(&mut self) -> f64 {
        let log_total_weight = self.normalize_weights();
        self.log_ml_estimate += log_total_weight - (self.num_particles as f64).ln();

        self.multinomial_resampling();

        let mut tmp_traces = vec![];
        for i in 0..self.num_particles {
            tmp_traces.push(self.traces[self.parents[i]].clone());
        }
        self.traces = tmp_traces;
        self.log_weights.fill(0.);
        log_total_weight
    }

    /// Return the current log marginal likelihood estimate from the particles.
    pub fn log_marginal_likelihood_estimate(&self) -> f64 {
        self.log_ml_estimate + logsumexp(&self.log_weights) - (self.num_particles as f64).ln()
    }
}