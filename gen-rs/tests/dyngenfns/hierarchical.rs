use std::sync::Weak;
use gen_rs::{DynTrie,Trace,DynGenFn,DynGenFnHandler,normal,bernoulli};


// models

fn _linear(state: &mut DynGenFnHandler<(),(f64,f64)>, _: ()) -> (f64,f64) {
    let a = state.sample_at(&normal, (0.,1.), "a");
    let b = state.sample_at(&normal, (0.,1.), "b");
    (a, b)
}
pub const linear: DynGenFn<(),(f64,f64)> = DynGenFn { func: _linear };

fn _quadratic(state: &mut DynGenFnHandler<(),(f64,f64,f64)>, _: ()) -> (f64,f64,f64) {
    let a = state.sample_at(&normal, (0.,1.), "a");
    let b = state.sample_at(&normal, (0.,1.), "b");
    let c = state.sample_at(&normal, (0.,1.), "c");
    (a, b, c)
}
pub const quadratic: DynGenFn<(),(f64,f64,f64)> = DynGenFn { func: _quadratic };

fn _hierarchical_model(state: &mut DynGenFnHandler<Vec<f64>,Vec<f64>>, xs: Vec<f64>) -> Vec<f64> {
    const noise: f64 = 0.1;
    if state.sample_at(&bernoulli, 0.7, "is_linear") {
        let coeffs = state.trace_at(&linear, (), "coeffs");
        xs.iter().enumerate().map(|(i, x)|
            state.sample_at(&normal, (coeffs.0 + coeffs.1 * x, noise), &format!("(y, {})", i)
        )).collect::<_>()
    } else {
        let coeffs = state.trace_at(&quadratic, (), "coeffs");
        xs.iter().enumerate().map(|(i, x)|
            state.sample_at(&normal, (coeffs.0 + coeffs.1 * x + coeffs.2 * x*x, noise), &format!("(y, {})", i)
        )).collect::<_>()
    }
}
pub const hierarchical_model: DynGenFn<Vec<f64>,Vec<f64>> = DynGenFn { func: _hierarchical_model };


// proposals

fn _add_or_remove_param_proposal(state: &mut DynGenFnHandler<(Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,()),()>,
                                 args: (Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,())) -> () {
    let trace = args.0.upgrade().unwrap();
    state.sample_at(&normal, (trace.data.read::<f64>("coeffs / a"), 0.025), "coeffs / a");
    state.sample_at(&normal, (trace.data.read::<f64>("coeffs / b"), 0.025), "coeffs / b");
    if !state.sample_at(&bernoulli, 0.5, "is_linear") {
        let prev_c = if trace.data.search("coeffs / c").is_some() {
            trace.data.read::<f64>("coeffs / c")
        } else {
            0.
        };
        state.sample_at(&normal, (prev_c, 0.025), "coeffs / c");
    }
}
pub const add_or_remove_param_proposal: DynGenFn<(Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,()),()> = DynGenFn { func: _add_or_remove_param_proposal };

fn _hierarchical_drift_proposal(state: &mut DynGenFnHandler<(Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,f64),()>,
                   args: (Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,f64)) -> () {
    let trace = args.0.upgrade().unwrap();
    let drift_std = args.1;
    state.sample_at(&normal, (trace.data.read::<f64>("coeffs / a"), drift_std), "coeffs / a");
    state.sample_at(&normal, (trace.data.read::<f64>("coeffs / b"), drift_std), "coeffs / b");
    if !trace.data.read::<bool>("is_linear") {
        state.sample_at(&normal, (trace.data.read::<f64>("coeffs / c"), drift_std), "coeffs / c");
    }
}
pub const hierarchical_drift_proposal: DynGenFn<(Weak<Trace<Vec<f64>,DynTrie,Vec<f64>>>,f64),()> = DynGenFn { func: _hierarchical_drift_proposal };


pub fn read_coeffs(trace: &Trace<Vec<f64>,DynTrie,Vec<f64>>) -> Vec<f64> {
    let a = trace.data.read::<f64>("coeffs / a");
    let b = trace.data.read::<f64>("coeffs / b");
    let is_linear = trace.data.read::<bool>("is_linear");
    if !is_linear {
        let c = trace.data.read::<f64>("coeffs / c");
        vec![a, b, c]
    } else {
        vec![a, b]
    }
}