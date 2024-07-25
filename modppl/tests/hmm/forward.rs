use nalgebra::{DVector, DMatrix};

pub fn hmm_forward_alg(
    prior: DVector<f64>,
    emission_dists: DMatrix<f64>,
    transition_dists: DMatrix<f64>,
    observations: &Vec<usize>
) -> f64 {
    assert_eq!(prior.nrows(), emission_dists.ncols());
    assert_eq!(prior.nrows(), transition_dists.ncols());
    assert_eq!(transition_dists.nrows(), transition_dists.ncols());
    let mut marginal_likelihood = 1.0;
    let mut alpha: DVector<f64> = prior.clone();
    for obs in observations.into_iter() {
        let likelihoods = emission_dists.row(*obs).transpose();
        let mut prev_posterior = alpha.component_mul(&likelihoods);
        let evidence = prev_posterior.sum();
        prev_posterior /= evidence;
        alpha = &transition_dists * prev_posterior;
        marginal_likelihood *= evidence;
    }
    marginal_likelihood
}