use std::fs::{write, create_dir_all};
use std::rc::Rc;
use rand::rngs::ThreadRng;
use nalgebra::{dvector,dmatrix};

use gen_rs::{GenerativeFunction,Trace,ChoiceBuffer,ChoiceHashMap};

mod pointed;
use pointed::types_2d::{Bounds,Point};
use pointed::{PointedModel, DriftProposal};

#[test]
fn test_metropolis_hastings() -> std::io::Result<()> {
    create_dir_all("../data")?;

    let mut rng = ThreadRng::default();
    const NUM_ITERS: u32 = 25000;

    let model = PointedModel { obs_cov: dmatrix![1., -3./5.; -3./5., 2.] };
    let proposal = DriftProposal { drift_cov: dmatrix![0.25, 0.; 0., 0.25] };
    let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
    let obs = dvector![0., 0.];

    let mut constraints = ChoiceHashMap::<Point>::new();
    constraints.set_value("obs", &Rc::new(obs));

    let mut trace = model.generate(&mut rng, bounds, constraints);
    for iter in 0..NUM_ITERS {
        dbg!(iter);
        let (new_trace, accepted) = gen_rs::mh(&mut rng, &model, trace, &proposal, bounds);
        dbg!(accepted);
        trace = new_trace;
        let data = trace.get_choices()["latent"].clone();
        let json = format!("[{},{}]", data[0], data[1]);
        write(format!("../data/mh_trace_{}.json", iter), json)?;
    }
    
    Ok(())
}
