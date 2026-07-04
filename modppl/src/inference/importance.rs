use crate::Real;
use crate::{categorical, logsumexp, Distribution, GenFn, Trace};
use rand::rngs::ThreadRng;

/// Performs inference for a `GenFn` via importance sampling.
///
/// Given a `model`, input arguments `model_args`, and `constraints`,
/// returns a tuple of:
/// 1. a vector of traces generated from `model` under the `constraints`.
/// 2. the log of the normalized weights using the internal proposal.
/// 3. the log marginal likelihood estimate of the `constraints` under the `model`.
pub fn importance_sampling<Args: Clone, Data: Clone, Ret>(
    model: &impl GenFn<Args, Data, Ret>,
    model_args: Args,
    constraints: Data,
    num_samples: u32,
) -> (Vec<Trace<Args, Data, Ret>>, Vec<Real>, Real) {
    let out = (0..num_samples)
        .map(|_| model.generate(model_args.clone(), constraints.clone()))
        .collect::<Vec<(Trace<Args, Data, Ret>, Real)>>();
    let log_total_weight = logsumexp(&out.iter().map(|(_, w)| *w).collect::<Vec<Real>>());
    let log_ml_estimate = log_total_weight - (num_samples as Real).ln();
    let log_normalized_weights = out
        .iter()
        .map(|(_, w)| w - log_total_weight)
        .collect::<Vec<Real>>();
    let traces = out.into_iter().map(|(tr, _)| tr).collect::<_>();
    (traces, log_normalized_weights, log_ml_estimate)
}

/// Performs inference for a `GenFn` via importance resampling.
///
/// Given a `model`, input arguments `model_args`, and `constraints`,
/// returns a tuple of:
/// 1. a vector of traces generated from `model` under the `constraints`.
/// 2. a resampled set of traces according to the normalized probabilities.
/// 3. the log marginal likelihood estimate of the `constraints` under the `model`.
pub fn importance_resampling<Args: Clone, Data: Clone, Ret>(
    model: &impl GenFn<Args, Data, Ret>,
    model_args: Args,
    constraints: Data,
    num_samples: u32,
    num_ret_samples: u32,
) -> (Vec<Trace<Args, Data, Ret>>, Vec<usize>, Real) {
    let (traces, weights, log_ml_estimate) =
        importance_sampling(model, model_args, constraints, num_samples);
    let mut rng = ThreadRng::default();
    let probs = weights.iter().map(|w| w.exp()).collect::<Vec<Real>>();
    let resampled_indices = (0..num_ret_samples)
        .map(|_| categorical.random(&mut rng, probs.clone()) as usize)
        .collect::<Vec<usize>>();
    (traces, resampled_indices, log_ml_estimate)
}
