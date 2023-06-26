use rand::rngs::ThreadRng;
use std::rc::Rc;
use crate::{
    mathutils::logsumexp,
    Trace,
    ChoiceBuffer, GenerativeFunction
};


pub fn importance_sampling<X,T,U: Trace<T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    model_args: Rc<X>,
    observations: impl ChoiceBuffer,
    num_samples: u32
) -> (Vec<U>, Vec<f64>, f64) {
    let traces = (0..num_samples)
        .map(|_| model.generate(rng, Rc::clone(&model_args), observations.clone()))
        .collect::<Vec<U>>();
    let log_total_weight = logsumexp(&traces.iter().map(|tr| tr.get_score()).collect::<Vec<f64>>());
    let log_ml_estimate = log_total_weight - (num_samples as f64).ln();
    let log_normalized_weights = traces.iter()
        .map(|tr| tr.get_score() - log_total_weight)
        .collect::<Vec<f64>>();
    return (traces, log_normalized_weights, log_ml_estimate)
}