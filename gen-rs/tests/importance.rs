// use std::fs::{write, create_dir_all};
// use std::rc::Rc;
// use rand::rngs::ThreadRng;
// use nalgebra::{dvector,dmatrix};

// use gen_rs::modeling::dists::{Distribution, categorical};
// use gen_rs::{Trace,ChoiceBuffer,ChoiceHashMap};

// mod pointed;
// use pointed::types_2d::{Bounds,Point};
// use pointed::{PointedModel, PointedTrace};

// #[test]
// fn test_importance() -> std::io::Result<()> {
//     create_dir_all("../data")?;

//     let mut rng = ThreadRng::default();
//     const NUM_SAMPLES: u32 = 100000;

//     let model = PointedModel { obs_cov: dmatrix![1., -3./5.; -3./5., 2.] };
//     let bounds = Bounds { xmin: -5., xmax: 5., ymin: -5., ymax: 5. };
//     let obs = dvector![0., 0.];

//     let mut constraints = ChoiceHashMap::<Point>::new();
//     constraints.set_value("obs", &Rc::new(obs));

//     let (traces, log_normalized_weights, log_ml_estimate) = 
//         gen_rs::importance_sampling(&mut rng, &model, bounds, constraints, NUM_SAMPLES);

//     dbg!(log_ml_estimate);

//     let data = traces.iter().map(|tr| tr.get_choices()["latent"].data.as_vec().to_vec()).collect::<Vec<Vec<f64>>>();
//     let json = serde_json::to_string(&data)?;
//     write("../data/initial_traces.json", json)?;

//     let probs = log_normalized_weights.iter()
//         .map(|w| w.exp())
//         .collect::<Vec<f64>>();
//     let traces = (0..NUM_SAMPLES/10)
//         .map(|_| categorical.random(&mut rng, probs.clone()))
//         .map(|idx| &traces[idx])
//         .collect::<Vec<&PointedTrace>>();
    
//     let data = traces.iter().map(|tr| tr.get_choices()["latent"].data.as_vec().to_vec()).collect::<Vec<Vec<f64>>>();
//     let json = serde_json::to_string(&data)?;
//     write("../data/resampled_traces.json", json)?;

//     Ok(())
// }