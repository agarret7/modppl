use std::fs::{write, create_dir_all};
use std::{
    sync::Arc,
    f64::consts::PI
};
use modppl::{Distribution,DynTrie,u01,normal,inference::ParticleSystem};
use nalgebra::dvector;
use rand::rngs::ThreadRng;

pub mod pointed_model;
use pointed_model::types_2d::{Bounds,Point};

pub mod dyngenfns;
use dyngenfns::spiral_model;


fn simulate_loop(rng: &mut ThreadRng, bounds: &Bounds, timesteps: i64) -> Vec<DynTrie>{
    let init_angle = u01(rng) * 2.*PI;

    let xrange = (bounds.xmax - bounds.xmin) as f64;
    let yrange = (bounds.ymax - bounds.ymin) as f64;
    let center = dvector![
        xrange / 2. + bounds.xmin,
        yrange / 2. + bounds.ymin
    ];
    let radius = f64::max(bounds.xmax - bounds.xmin, bounds.ymax - bounds.ymin) / 5.;
    
    let mut observations = vec![];
    let perturb_means = (0..timesteps).filter(|_| u01(rng) < 0.3).collect::<Vec<_>>();
    for t in 0..timesteps {
        let mut deformation = 0.;
        for perturb_t in &perturb_means {
            deformation += normal.logpdf(&(t as f64), (perturb_t.clone() as f64, 1.)).exp()
        }
        let r = radius + deformation;
        let t = 2.*PI*(t as f64) / timesteps as f64;
        let obs = dvector![
            center[0] + r*(t + init_angle).cos(),
            center[1] + r*(t + init_angle).sin()
        ];
        let mut constraints = DynTrie::new();
        constraints.observe("obs", Arc::new(obs));
        observations.push(constraints);
    }
    observations
}

#[test]
fn test_smc() -> std::io::Result<()> {
    create_dir_all("../data")?;

    let mut rng = ThreadRng::default();
    const NUM_TIMESTEPS: i64 = 20;
    const NUM_PARTICLES: usize = 500;

    let bounds = Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1.};
    let data = simulate_loop(&mut rng, &bounds, NUM_TIMESTEPS);
    let obs = data.iter().map(|t| t.read::<Point>("obs")).collect::<Vec<Point>>();
    let obs_strs = obs.iter().map(|obs| format!("[{},{}]", obs[0], obs[1])).collect::<Vec<String>>();
    let json = "[".to_owned() + &obs_strs.join(", ") + "]";
    write(format!("../data/smc_obs.json"), json)?;

    let mut filter = ParticleSystem::new(spiral_model, NUM_PARTICLES, rng);
    let mut data_it = data.into_iter();
    filter.init_step(dvector![0.,0.], vec![data_it.next().unwrap()]);

    let states = filter.traces.iter().map(|vtr| vtr.retv.as_ref().unwrap().last().unwrap().clone()).collect::<Vec<_>>();
    let state_strs = states.iter().map(|latent| format!("[{},{}]", latent[0], latent[1])).collect::<Vec<String>>();
    let json = "[".to_owned() + &state_strs.join(", ") + "]";
    write(format!("../data/smc_traces_before_resample_0.json"), json)?;
    filter.resample();

    let states = filter.traces.iter().map(|vtr| vtr.retv.as_ref().unwrap().last().unwrap().clone()).collect::<Vec<_>>();
    let state_strs = states.iter().map(|latent| format!("[{},{}]", latent[0], latent[1])).collect::<Vec<String>>();
    let json = "[".to_owned() + &state_strs.join(", ") + "]";
    write(format!("../data/smc_traces_{}.json", 0), json)?;

    for (t,constraints) in data_it.enumerate() {
        filter = filter.step(vec![constraints]);
        let states = filter.traces.iter().map(|vtr| vtr.retv.as_ref().unwrap().last().unwrap().clone()).collect::<Vec<_>>();
        let state_strs = states.iter().map(|latent| format!("[{},{}]", latent[0], latent[1])).collect::<Vec<String>>();
        let json = "[".to_owned() + &state_strs.join(", ") + "]";
        write(format!("../data/smc_traces_before_resample_{}.json", t+1), json)?;
        filter.resample();
        let states = filter.traces.iter().map(|vtr| vtr.retv.as_ref().unwrap().last().unwrap().clone()).collect::<Vec<_>>();
        let state_strs = states.iter().map(|latent| format!("[{},{}]", latent[0], latent[1])).collect::<Vec<String>>();
        let json = "[".to_owned() + &state_strs.join(", ") + "]";
        write(format!("../data/smc_traces_{}.json", t+1), json)?;
    }

    Ok(())
}