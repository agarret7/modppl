use std::fs::{write, create_dir_all};
use gen_rs::prelude::*;


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
fn hierarchical_model(xs: Vec<f64>) -> Vec<f64> {
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
fn add_or_remove_param_proposal(tr: Weak<DynTrace<Vec<f64>,Vec<f64>>>) {
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
fn hierarchical_drift_proposal(tr: Weak<DynTrace<Vec<f64>,Vec<f64>>>, drift_std: f64) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>("coeffs/a"), drift_std) %= "coeffs/a";
    normal(tr.data.read::<f64>("coeffs/b"), drift_std) %= "coeffs/b";
    if !tr.data.read::<bool>("is_linear") {
        normal(tr.data.read::<f64>("coeffs/c"), drift_std) %= "coeffs/c";
    }
});

pub fn read_coeffs(trace: &DynTrace<Vec<f64>,Vec<f64>>) -> Vec<f64> {
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

#[test]
pub fn test_infer_hierarchical() -> std::io::Result<()> {
    create_dir_all("../data")?;

    let mut rng = ThreadRng::default();

    let xs = vec![-5.,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];

    let mut observations = DynTrie::new();
    let (a, b, c) = (0.3, 0.4, 0.5);
    let ys = xs.iter().map(|x|
        a + b*x + c*x*x + normal.random(&mut rng, (0., 0.1))
    ).collect::<Vec<f64>>();
    write("../data/hierarchical_data.json", format!("[{:?}, {:?}]", xs, ys))?;
    ys.into_iter().enumerate().for_each(|(i, y)| { observations.observe(&format!("(y, {})", i), Rc::new(y)); });

    let mut trace = hierarchical_model.generate(xs, observations).0;
    let mut all_coeffs = vec![];
    for _ in 0..100 {
        let (new_trace, _) = mh(&hierarchical_model, trace, &add_or_remove_param_proposal, ());
        trace = new_trace;
        all_coeffs.push(read_coeffs(&trace));
        for _ in 0..3 {
            let (new_trace, _) = mh(&hierarchical_model, trace, &hierarchical_drift_proposal, 0.1);
            trace = new_trace;
            all_coeffs.push(read_coeffs(&trace));
        }
        for _ in 0..10 {
            let (new_trace, _) = mh(&hierarchical_model, trace, &hierarchical_drift_proposal, 0.01);
            trace = new_trace;
            all_coeffs.push(read_coeffs(&trace));
        }
        write("../data/hierarchical_model.json", format!("{:?}", all_coeffs))?;
    }
    Ok(())
}