use std::fs::{write, create_dir_all};
use std::rc::Rc;
use rand::rngs::ThreadRng;

use gen_rs::modeling::dists::{Distribution, categorical};
use gen_rs::{Trace,ChoiceBuffer,ChoiceHashMap};
use gen_rs::types_2d::{Bounds,Point};
use pointed::{PointedModel, PointedTrace};

pub mod pointed;

#[test]
fn test_importance() -> std::io::Result<()> {
    create_dir_all("data")?;

    let mut rng = ThreadRng::default();
    const NUM_SAMPLES: u32 = 100000;

    let model = &PointedModel { obs_std: 0.25 };
    let bounds = Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };
    let obs = Point { x: 0., y: 0. };

    let mut constraints = ChoiceHashMap::<Point>::new();
    constraints.set_value("obs", &Rc::new(obs));

    let (traces, log_normalized_weights, log_ml_estimate) = 
        gen_rs::importance_sampling(&mut rng, model, Rc::new(bounds), constraints, NUM_SAMPLES);

    dbg!(log_ml_estimate);

    let data = traces.iter().map(|tr| *tr.get_choices()["latent"]).collect::<Vec<Point>>();
    let json = serde_json::to_string(&data)?;
    write("data/initial_traces.json", json)?;

    let probs = &log_normalized_weights.iter()
        .map(|w| (w - log_ml_estimate).exp())
        .collect::<Vec<f32>>();
    let traces = (0..NUM_SAMPLES/10)
        .map(|_| categorical.random(&mut rng, probs))
        .map(|idx| &traces[idx])
        .collect::<Vec<&PointedTrace>>();
    
    let data = traces.iter().map(|tr| *tr.get_choices()["latent"]).collect::<Vec<Point>>();
    let json = serde_json::to_string(&data)?;
    write("data/resampled_traces.json", json)?;

    Ok(())
}