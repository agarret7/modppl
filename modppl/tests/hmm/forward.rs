use modppl::Real;
use nalgebra::{DMatrix, DVector};

pub fn hmm_forward_alg(
    prior: DVector<Real>,
    emission_dists: DMatrix<Real>,
    transition_dists: DMatrix<Real>,
    observations: &Vec<usize>,
) -> Real {
    assert_eq!(prior.nrows(), emission_dists.ncols());
    assert_eq!(prior.nrows(), transition_dists.ncols());
    assert_eq!(transition_dists.nrows(), transition_dists.ncols());
    let mut marginal_likelihood = 1.0;
    let mut alpha: DVector<Real> = prior.clone();
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
