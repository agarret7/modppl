use std::sync::Weak;
use modppl::prelude::*;


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

dyngen!(
fn linear() -> (f64,f64) {
    let a = normal(0.,1.) %= "a";
    let b = normal(0.,1.) %= "b";
    (a, b)
});

dyngen!(
fn quadratic() -> (f64,f64,f64) {
    let a = normal(0.,1.) %= "a";
    let b = normal(0.,1.) %= "b";
    let c = normal(0.,1.) %= "c";
    (a, b, c)
});

dyngen!(
pub fn hierarchical_model(xs: Vec<f64>) -> Vec<f64> {
    const noise: f64 = 0.1;
    if bernoulli(0.7) %= "is_linear" {
        let coeffs = linear() /= "coeffs";
        xs.iter().enumerate().map(|(i, x)| 
            normal(coeffs.0 + coeffs.1 * x, noise) %= &format!("(y, {})", i)
        ).collect::<_>()
    } else {
        let coeffs = quadratic() /= "coeffs";
        xs.iter().enumerate().map(|(i, x)| 
            normal(coeffs.0 + coeffs.1 * x + coeffs.2 * x * x, noise) %= &format!("(y, {})", i)
        ).collect::<_>()
    }
});

dyngen!(
pub fn add_or_remove_param_proposal(tr: Weak<DynTrace<Vec<f64>,Vec<f64>>>) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>("coeffs/a"), 0.025) %= "coeffs/a";
    normal(tr.data.read::<f64>("coeffs/b"), 0.025) %= "coeffs/b";
    if !(bernoulli(0.5) %= "is_linear") {
        let prev_c = if tr.data.search("coeffs/c").is_some() {
            tr.data.read::<f64>("coeffs/c")
        } else {
            0.
        };
        normal(prev_c, 0.025) %= "coeffs/c";
    }
});

dyngen!(
pub fn hierarchical_drift_proposal(tr: Weak<DynTrace<Vec<f64>,Vec<f64>>>, drift_std: f64) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>("coeffs/a"), drift_std) %= "coeffs/a";
    normal(tr.data.read::<f64>("coeffs/b"), drift_std) %= "coeffs/b";
    if !tr.data.read::<bool>("is_linear") {
        normal(tr.data.read::<f64>("coeffs/c"), drift_std) %= "coeffs/c";
    }
});