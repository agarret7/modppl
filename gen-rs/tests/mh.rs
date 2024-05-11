use std::any::Any;
use std::sync::Arc;
use std::fs::{write, create_dir_all};
use rand::rngs::ThreadRng;
use nalgebra::{DVector, dvector, dmatrix};

use gen_rs::{GenFn, Trie, Distribution, normal, mh};

mod pointed_model;
use pointed_model::types_2d::Bounds;
use pointed_model::{PointedModel, DriftProposal};

mod dyngenfns;
use dyngenfns::{pointed_2d_model, pointed_2d_drift_proposal};
use dyngenfns::{hierarchical_model, read_coeffs,
    hierarchical_drift_proposal,
    add_or_remove_param_proposal
};


#[test]
fn test_metropolis_hastings_handcoded() -> std::io::Result<()> {
    create_dir_all("../data")?;

    const NUM_ITERS: u32 = 25000;

    let model = PointedModel { obs_cov: dmatrix![1., -3./5.; -3./5., 2.] };
    let proposal = DriftProposal { drift_cov: dmatrix![0.25, 0.; 0., 0.25] };
    let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
    let obs = dvector![0., 0.];

    let constraints = (None, Some(obs));

    let (mut trace, _) = model.generate(bounds, constraints);
    for iter in 0..NUM_ITERS {
        dbg!(iter);
        let (new_trace, accepted) = gen_rs::mh(&model, trace, &proposal, ());
        dbg!(accepted);
        trace = new_trace;
        let data = trace.data.0.clone().unwrap();
        let json = format!("[{},{}]", data[0], data[1]);
        write(format!("../data/mh_trace_{}.json", iter), json)?;
    }
    
    Ok(())
}


#[test]
pub fn test_metropolis_hastings_dyngenfn() -> std::io::Result<()> {
    create_dir_all("../data")?;

    const NUM_ITERS: u32 = 25000;

    let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
    let obs = dvector![0., 0.];

    let mut observations = Trie::new();
    observations.observe("obs", Arc::new(obs) as Arc<dyn Any + Send + Sync>);

    let mut trace = pointed_2d_model.generate((bounds, dmatrix![1., -3./5.; -3./5., 2.]), observations).0;
    for iter in 0..NUM_ITERS {
        dbg!(iter);
        let (new_trace, accepted) = mh(&pointed_2d_model, trace, &pointed_2d_drift_proposal, dmatrix![0.25, 0.; 0., 0.25]);
        dbg!(accepted);
        trace = new_trace;
        let data = trace.data.read::<DVector<f64>>("latent");
        let json = format!("[{},{}]", data[0], data[1]);
        write(format!("../data/mh_trace_{}.json", iter), json)?;
    }

    Ok(())
}


#[test]
pub fn test_metropolis_hastings_hierarchical() -> std::io::Result<()> {
    create_dir_all("../data")?;

    let mut rng = ThreadRng::default();

    let xs = vec![-5.,-4.,-3.,-2.,-1.,0.,1.,2.,3.,4.,5.];

    let mut observations = Trie::new();
    let (a, b, c) = (0.3, 0.4, 0.5);
    let ys = xs.iter().map(|x|
        a + b*x + c*x*x + normal.random(&mut rng, (0., 0.1))
    ).collect::<Vec<f64>>();
    write("../data/hierarchical_data.json", format!("[{:?}, {:?}]", xs, ys))?;
    ys.into_iter().enumerate().for_each(|(i, y)| { observations.observe(&format!("(y, {})", i), Arc::new(y) as Arc<dyn Any + Send + Sync>); });

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