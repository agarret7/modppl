use std::{any::Any,rc::Rc};
use std::fs::{write,create_dir_all};
use rand::rngs::ThreadRng;
use nalgebra::{dvector,dmatrix};

use gen_rs::{Trace,Trie,Distribution,importance_sampling,normal,categorical};

mod pointed_model;
use pointed_model::types_2d::Bounds;
use pointed_model::{PointedTrace,PointedModel};

mod triefns;
use triefns::{line_model, hierarchical_model};


#[test]
fn test_importance_handcoded() -> std::io::Result<()> {
    create_dir_all("../data")?;

    const NUM_SAMPLES: u32 = 10000;
    let mut rng = ThreadRng::default();

    let model = PointedModel { obs_cov: dmatrix![1., -3./5.; -3./5., 2.] };
    let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
    let obs = dvector![0., 0.];

    let constraints = (None, Some(obs));

    let (traces, log_normalized_weights, log_ml_estimate) = 
        gen_rs::importance_sampling(&model, bounds, constraints, NUM_SAMPLES);

    dbg!(log_ml_estimate);

    let data = traces.iter().map(|tr| tr.data.0.as_ref().unwrap().data.as_vec().to_vec()).collect::<Vec<Vec<f64>>>();
    let json = serde_json::to_string(&data)?;
    write("../data/initial_traces.json", json)?;

    let probs = log_normalized_weights.iter()
        .map(|w| w.exp())
        .collect::<Vec<f64>>();
    let traces = (0..NUM_SAMPLES/10)
        .map(|_| categorical.random(&mut rng, probs.clone()))
        .map(|idx| &traces[idx])
        .collect::<Vec<&PointedTrace>>();
    
    let data = traces.iter().map(|tr| tr.data.0.as_ref().unwrap().data.as_vec().to_vec()).collect::<Vec<Vec<f64>>>();
    let json = serde_json::to_string(&data)?;
    write("../data/resampled_traces.json", json)?;

    Ok(())
}


#[test]
pub fn test_importance_triefn() {
    const NUM_SAMPLES: u32 = 10000;

    let mut rng = ThreadRng::default();

    let xs = vec![-5., -4., -3., -2., -1., 0., 1., 2., 3., 4., 5.];
    let mut observations = Trie::new();
    xs.iter()
        .enumerate()
        .for_each(|(i, x)| {
            observations.insert_leaf_node(
                Box::leak(format!("ys => {}", i).into_boxed_str()),
                Rc::new(0.5*x - 1. + normal.random(&mut rng, (0., 0.1))) as Rc<dyn Any>);
            });
    let (traces, log_normalized_weights, lml_estimate) = importance_sampling(&line_model, xs, Trie::from_unweighted(observations), NUM_SAMPLES);

    let probs = log_normalized_weights.iter()
        .map(|w| w.exp())
        .collect::<Vec<f64>>();
    let traces = (0..NUM_SAMPLES/10)
        .map(|_| categorical.random(&mut rng, probs.clone()))
        .map(|idx| &traces[idx])
        .collect::<Vec<&Trace<_,_,_>>>();
    for i in 0..20 {
        println!("Trace {}", i);
        println!("slope = {}", &traces[i].data.get_leaf_node("slope").unwrap().0.clone().downcast::<f64>().ok().unwrap());
        println!("intercept = {}", &traces[i].data.get_leaf_node("intercept").unwrap().0.clone().downcast::<f64>().ok().unwrap());
    }
    dbg!(lml_estimate);
}


#[test]
pub fn test_importance_hierarchical() -> std::io::Result<()> {
    const NUM_SAMPLES: u32 = 10000;
    let mut rng = ThreadRng::default();

    let xs = vec![-5.,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];

    let mut observations = Trie::new();
    let (a, b, c) = (0.3, 0.4, 0.5);
    let ys = xs.iter().map(|x|
        a + b*x + c*x*x + normal.random(&mut rng, (0., 0.1))
    ).collect::<Vec<f64>>();
    write("../data/hierarchical_data.json", format!("[{:?}, {:?}]", xs, ys))?;
    ys.into_iter().enumerate().for_each(|(i, y)| { observations.insert_leaf_node(&format!("(y, {})", i), Rc::new(y) as Rc<dyn Any>); });

    let (traces, log_normalized_weights, lml_estimate) =
        importance_sampling(&hierarchical_model, xs, Trie::from_unweighted(observations), NUM_SAMPLES);

    let probs = log_normalized_weights.iter()
        .map(|w| w.exp())
        .collect::<Vec<f64>>();
    let traces = (0..NUM_SAMPLES/10)
        .map(|_| categorical.random(&mut rng, probs.clone()))
        .map(|idx| &traces[idx])
        .collect::<Vec<&Trace<_,_,_>>>();
    let mut all_coeffs = vec![];
    for i in 0..200 {
        println!("Trace {}", i);
        let is_linear = &traces[i].data.get_leaf_node("is_linear").unwrap().0.clone().downcast::<bool>().ok().unwrap();
        println!("is_linear = {}", is_linear);
        let a = traces[i].data.get_leaf_node("coeffs => a").unwrap().0.clone().downcast::<f64>().ok().unwrap();
        let b = traces[i].data.get_leaf_node("coeffs => b").unwrap().0.clone().downcast::<f64>().ok().unwrap();
        let coeffs = if !*is_linear.as_ref() {
            let c = traces[i].data.get_leaf_node("coeffs => c").unwrap().0.clone().downcast::<f64>().ok().unwrap();
            vec![a, b, c]
        } else {
            vec![a, b]
        };
        println!("coeffs: {:?}", coeffs);
        all_coeffs.push(coeffs);
    }
    write("../data/hierarchical_model.json", format!("{:?}", all_coeffs))?;

    dbg!(lml_estimate);
    Ok(())
}