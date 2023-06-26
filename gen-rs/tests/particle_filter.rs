// use std::fs::{write, create_dir_all};
// use std::rc::Rc;
// use rand::rngs::ThreadRng;

// use gen_rs::modeling::dists::{Distribution, categorical};
// use gen_rs::{Trace,ChoiceBuffer,ChoiceHashMap};
// use pf::ParticleFilterModel;

// pub mod pointed;

// #[test]
// fn test_particle_filter() -> std::io::Result<()> {
//     create_dir_all("data")?;

//     let mut rng = ThreadRng::default();
//     const NUM_SAMPLES: u32 = 100000;

//     let model = &ParticleFilterModel { };

//     Ok(())
// }