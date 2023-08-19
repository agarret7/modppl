use crate::{mathutils::logsumexp, Trace, GenFn};


/// Performs inference for a `GenFn` via importance sampling.
/// 
/// Given a `model`, input arguments `model_args`, and `constraints`,
/// returns a tuple of:
/// 1. a vector of traces generated from `model` under the `constraints`.
/// 2. the log of the normalized weights using the internal proposal.
/// 3. the log marginal likelihood estimate of the `constraints` under the `model`.
pub fn importance_sampling<Args: Clone,Data: Clone,Ret>(
    model: &impl GenFn<Args,Data,Ret>,
    model_args: Args,
    constraints: Data,
    num_samples: u32
) -> (Vec<Trace<Args,Data,Ret>>, Vec<f64>, f64) {
    let out = (0..num_samples)
        .map(|_| model.generate(model_args.clone(), constraints.clone()))
        .collect::<Vec<(Trace<Args,Data,Ret>,f64)>>();
    let log_total_weight = logsumexp(&out.iter().map(|(_, w)| *w).collect::<Vec<f64>>());
    let log_ml_estimate = log_total_weight - (num_samples as f64).ln();
    let log_normalized_weights = out.iter()
        .map(|(_, w)| w - log_total_weight)
        .collect::<Vec<f64>>();
    return (out.into_iter().map(|(tr, _)| tr).collect::<_>(), log_normalized_weights, log_ml_estimate)
}