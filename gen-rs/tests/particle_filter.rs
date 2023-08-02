// use rand::rngs::ThreadRng;
// use nalgebra::{dvector,dmatrix};

// use gen_rs::{GfDiff, ChoiceHashMap};
// use gen_rs::ParticleSystem;
// use common_macros::hash_map;

// mod hmm;



// #[test]
// fn test_hmm_forward_algorithm() {
//     let prior = dvector![0.4, 0.6];
//     let emission_dists = dmatrix![0.1, 0.9; 0.7, 0.3].transpose();
//     let transition_dists = dmatrix![0.5, 0.5; 0.2, 0.8].transpose();
//     let obs = vec![1, 0];

//     let mut true_marginal_likelihood = 0.0;
//     // z = [0, 0]
//     true_marginal_likelihood += prior[0]                * emission_dists[(obs[0] as usize, 0)]
//                               * transition_dists[(0,0)] * emission_dists[(obs[1] as usize, 0)];
//     // z = [0, 1]
//     true_marginal_likelihood += prior[0]                * emission_dists[(obs[0] as usize, 0)]
//                               * transition_dists[(1,0)] * emission_dists[(obs[1] as usize, 1)];
//     // z = [1, 0]
//     true_marginal_likelihood += prior[1]                * emission_dists[(obs[0] as usize, 1)]
//                               * transition_dists[(0,1)] * emission_dists[(obs[1] as usize, 0)];
//     // z = [1, 1]
//     true_marginal_likelihood += prior[1]                * emission_dists[(obs[0] as usize, 1)]
//                               * transition_dists[(1,1)] * emission_dists[(obs[1] as usize, 1)];

//     let empirical_marginal_likelihood = hmm::hmm_forward_alg(prior, emission_dists, transition_dists, &obs);
//     approx::assert_abs_diff_eq!(empirical_marginal_likelihood, true_marginal_likelihood, epsilon = 1e-16);
// }

// #[test]
// fn test_particle_filter() -> std::io::Result<()> {
//     let rng = ThreadRng::default();
//     const NUM_PARTICLES: usize = 10000;

//     let prior = dvector![0.2, 0.3, 0.5];
//     let emission_matrix = dmatrix![
//         0.1, 0.2, 0.7;
//         0.2, 0.7, 0.1;
//         0.7, 0.2, 0.1
//     ].transpose();
//     let transition_matrix = dmatrix![
//         0.4, 0.4, 0.2;
//         0.2, 0.3, 0.5;
//         0.9, 0.05, 0.05
//     ].transpose();
//     let params = hmm::HMMParams::new(prior.clone(), emission_matrix.clone(), transition_matrix.clone());

//     let model = hmm::HMM::new(params);

//     let data = vec![0, 0, 1, 2];
//     let expected = hmm::hmm_forward_alg(prior, emission_matrix, transition_matrix, &data).ln();

//     let mut filter = ParticleSystem::new(model, NUM_PARTICLES, rng);

//     let store = hmm::ParamStore { };
//     let mut data_it = data.into_iter();
//     filter.init_step(store, ChoiceHashMap::from_hashmap(hash_map!("1 => observation" => data_it.next().unwrap())));
//     println!("T = {}", 1);

//     for (t, obs) in data_it.enumerate() {
//         println!("T = {}", t+2);  // time is 1-indexed and init_step used "1 => observation" (1 + 1 = 2)
//         let addr: &'static str = Box::leak(format!("{} => observation", t+2).into_boxed_str());
//         let constraints = ChoiceHashMap::from_hashmap(hash_map!(addr => obs));
//         filter.step(GfDiff::Extend, constraints);
//         let ess = filter.effective_sample_size();
//         dbg!(ess);
//         let log_weight = filter.resample();
//         dbg!(log_weight);
//     }
//     let lml_estimate = filter.log_marginal_likelihood_estimate();

//     approx::assert_abs_diff_eq!(lml_estimate, expected, epsilon = 0.02);

//     Ok(())
// }