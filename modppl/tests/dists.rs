use nalgebra::{dmatrix, dvector};
use std::collections::HashMap;

use approx;
use modppl::{
    bernoulli, beta, binomial, categorical, cauchy, dirichlet, exponential, gamma, geometric,
    inv_gamma, laplace, mvnormal, normal, poisson, uniform, uniform_discrete, Distribution, Real,
};
use rand::rngs::ThreadRng;

#[cfg(feature = "f32")]
const LOGPDF_EPSILON: Real = 1e-4;

#[cfg(not(feature = "f32"))]
const LOGPDF_EPSILON: Real = 1e-12;

fn mean(v: &[Real]) -> Real {
    v.iter().sum::<Real>() / v.len() as Real
}

fn variance(v: &[Real]) -> Real {
    let c = mean(v);
    v.iter().map(|x| (*x - c) * (*x - c)).sum::<Real>() / (v.len() as Real - 1.)
}

fn standard_deviation(v: &[Real]) -> Real {
    variance(v).sqrt()
}

#[test]
fn test_bernoulli() {
    let mut rng = ThreadRng::default();

    let true_p = 0.11;
    assert_eq!(bernoulli.logpdf(&true, true_p), true_p.ln());
    assert_eq!(bernoulli.logpdf(&false, true_p), (1. - true_p).ln());
    let samples = (0..50000)
        .map(|_| bernoulli.random(&mut rng, 0.11))
        .collect::<Vec<bool>>();

    let empirical_true = samples.iter().filter(|&&x| x).collect::<Vec<_>>().len();
    let empirical_false = samples.iter().filter(|&&x| !x).collect::<Vec<_>>().len();
    let empirical_freq = empirical_true as Real / empirical_false as Real;
    approx::assert_abs_diff_eq!(empirical_freq, true_p, epsilon = 0.02);
}

#[test]
fn test_uniform() {
    let mut rng = ThreadRng::default();

    // continuous
    let params = (0.5, 3.14);
    let (a, b) = params;
    let true_p = 1. / (b - a);
    assert_eq!(uniform.logpdf(&0.9, params), true_p.ln());
    assert_eq!(uniform.logpdf(&2.1, params), true_p.ln());
    assert_eq!(uniform.logpdf(&0.4, params), modppl::Real::NEG_INFINITY);
    let num_samples = 50000;
    let samples = (0..num_samples)
        .map(|_| uniform.random(&mut rng, params))
        .collect::<Vec<Real>>();
    let num_bins = 100;
    let mut hist = vec![0; num_bins];

    let expected_samples_per_bin = num_samples / num_bins;
    for x in samples {
        let i = ((x - a) / (b - a) * num_bins as Real).trunc() as usize;
        hist[i] += 1;
    }
    for bin_count in hist {
        approx::assert_abs_diff_eq!(bin_count, expected_samples_per_bin, epsilon = 150);
    }

    // discrete
    let params = (8, 130);
    let (a, b) = params;
    let true_p = 1. / (b - a + 1) as Real;
    assert_eq!(uniform_discrete.logpdf(&9, params), true_p.ln());
    assert_eq!(uniform_discrete.logpdf(&130, params), true_p.ln());
    assert_eq!(
        uniform_discrete.logpdf(&140, params),
        modppl::Real::NEG_INFINITY
    );
    let num_samples = 50000;
    let samples = (0..num_samples)
        .map(|_| uniform_discrete.random(&mut rng, params))
        .collect::<Vec<i64>>();
    let num_bins = 5;
    let mut hist = vec![0; num_bins];

    let expected_samples_per_bin = num_samples / num_bins;
    for x in samples {
        let i = ((x - a) as Real / (b - a + 1) as Real * num_bins as Real) as usize;
        hist[i] += 1;
    }
    for bin_count in hist {
        approx::assert_abs_diff_eq!(bin_count, expected_samples_per_bin, epsilon = 750);
    }
}

#[test]
fn test_categorical() {
    let mut rng = ThreadRng::default();
    let labels = vec!["a", "b", "c", "d", "e", "f"];
    let probs = vec![0.1, 0.3, 0.2, 0.1, 0.05, 0.25];
    let num_samples = 50000;
    let sample_indices = (0..num_samples)
        .map(|_| categorical.random(&mut rng, probs.clone()))
        .collect::<Vec<i64>>();

    let samples = sample_indices
        .iter()
        .map(|idx| labels[*idx as usize])
        .collect::<Vec<&str>>();

    let mut count = HashMap::new();

    for item in samples.iter() {
        *count.entry(item).or_insert(0) += 1;
    }
    for (i, gt_freq) in (0..6).zip(probs.iter()) {
        let freq = count[&labels[i]] as Real / num_samples as Real;
        approx::assert_abs_diff_eq!(freq, gt_freq, epsilon = 0.01);
    }
}

#[test]
fn test_normal() {
    let mut rng = ThreadRng::default();

    let true_mu = 1.64;
    let true_std = 0.025;

    let samples = (0..50000)
        .map(|_| normal.random(&mut rng, (true_mu, true_std)))
        .collect::<Vec<Real>>();

    let empirical_mu = mean(&samples);
    let empirical_std = standard_deviation(&samples);
    approx::assert_abs_diff_eq!(empirical_mu, true_mu, epsilon = 0.001);
    approx::assert_abs_diff_eq!(empirical_std, true_std, epsilon = 0.001);

    let x = 1.4;
    let mu = 0.9;
    let std = 0.5;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -0.7257913526447272, epsilon = LOGPDF_EPSILON);

    let x = 2.8;
    let mu = 1.8;
    let std = 1.;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -1.4189385332046727, epsilon = LOGPDF_EPSILON);

    let x = -3.14;
    let mu = 8.;
    let std = 20.;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -4.069795306758664, epsilon = LOGPDF_EPSILON);
}

#[test]
fn test_mvnormal() {
    let mut rng = ThreadRng::default();

    let true_mu = dvector![-1.5, 3.2];
    let true_cov = dmatrix![1.,-3./5.;-3./5.,2.];
    let params = (true_mu.clone(), true_cov.clone());

    let samples = (0..50000)
        .map(|_| {
            mvnormal
                .random(&mut rng, params.clone())
                .data
                .as_vec()
                .to_vec()
        })
        .collect::<Vec<Vec<Real>>>();
    let sample_xs = samples.iter().map(|p| p[0]).collect::<Vec<Real>>();
    let sample_ys = samples.iter().map(|p| p[1]).collect::<Vec<Real>>();
    let e_mu_x = mean(&sample_xs);
    let e_mu_y = mean(&sample_ys);
    let e_mu = dvector![e_mu_x, e_mu_y];
    approx::assert_abs_diff_eq!(e_mu, true_mu, epsilon = 0.05);
    let e_var_x = variance(&sample_xs);
    let e_var_y = variance(&sample_ys);
    let e_cov_xy = sample_xs
        .iter()
        .zip(sample_ys)
        .map(|(x, y)| (x - true_mu[0]) * (y - true_mu[1]))
        .sum::<Real>()
        / samples.len() as Real;
    let e_cov = dmatrix![e_var_x, e_cov_xy; e_cov_xy, e_var_y];
    approx::assert_abs_diff_eq!(e_cov, true_cov, epsilon = 0.05);

    let x = dvector![1.1, 5.8];
    let mu = dvector![1.3, 5.6];
    let cov = dmatrix![1., -0.81; -0.81, 2.5];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -2.1642100746383357, epsilon = LOGPDF_EPSILON);

    let x = dvector![30.1, -46.8];
    let mu = dvector![0., 6.];
    let cov = dmatrix![496., 0.13; 0.13, 500.];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -11.750458919763666, epsilon = LOGPDF_EPSILON);

    let x = dvector![1.2, 5.1, -7.8];
    let mu = dvector![1.4, 5.0, -7.4];
    let cov = dmatrix![1., 0.1, 0.9; 0.1, 1.3, 0.4; 0.9, 0.4, 1.75];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -2.873267436425841, epsilon = LOGPDF_EPSILON);
}

#[test]
pub fn test_geometric() {
    approx::assert_abs_diff_eq!(
        -1.3862943611198906,
        geometric.logpdf(&1, 0.5),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -19.580317734458244,
        geometric.logpdf(&5, 0.98),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -5.6202541071917365,
        geometric.logpdf(&101, 0.01),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_poisson() {
    approx::assert_abs_diff_eq!(
        -1.6328763858683835,
        poisson.logpdf(&3, 4.0),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -4.2601662022412240,
        poisson.logpdf(&5, 1.5),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -5.969204868031767,
        poisson.logpdf(&52, 36.11),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_beta() {
    approx::assert_abs_diff_eq!(
        -0.364406011717066,
        beta.logpdf(&0.3, (0.5, 0.5)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -0.06055443631298263,
        beta.logpdf(&0.7, (1.5, 2.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -0.36440601171706609,
        beta.logpdf(&0.3, (0.5, 0.5)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_gamma() {
    approx::assert_abs_diff_eq!(
        -1.414334369005868,
        gamma.logpdf(&1.7, (1.23, 1.46)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -3.4049256003700052,
        gamma.logpdf(&8.4, (4.5, 1.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -528.8122715889206,
        gamma.logpdf(&0.03, (50.0, 70.0)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_inv_gamma() {
    approx::assert_abs_diff_eq!(
        -1.0191707469882739,
        inv_gamma.logpdf(&1.5, (2.0, 3.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        0.07944154167983569,
        inv_gamma.logpdf(&0.5, (3.0, 1.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -0.8734566196629605,
        inv_gamma.logpdf(&0.8, (5.0, 2.0)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_binomial() {
    approx::assert_abs_diff_eq!(
        -2.143980062817407,
        binomial.logpdf(&3, (10, 0.5)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -1.7833747196936622,
        binomial.logpdf(&0, (5, 0.3)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -1.1157177565710488,
        binomial.logpdf(&5, (5, 0.8)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_exponential() {
    approx::assert_abs_diff_eq!(
        -2.3068528194400547,
        exponential.logpdf(&1.5, 2.0),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -0.4013877113318902,
        exponential.logpdf(&0.5, 3.0),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -2.802585092994046,
        exponential.logpdf(&5.0, 0.1),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_laplace() {
    approx::assert_abs_diff_eq!(
        -1.6931471805599453,
        laplace.logpdf(&1.0, (0.0, 1.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -1.8862943611198906,
        laplace.logpdf(&2.5, (1.5, 2.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -3.0986122886681098,
        laplace.logpdf(&-3.0, (0.0, 1.5)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_cauchy() {
    approx::assert_abs_diff_eq!(
        -1.8378770664093453,
        cauchy.logpdf(&1.0, (0.0, 1.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -2.0610206177235553,
        cauchy.logpdf(&2.0, (1.0, 2.0)),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        -3.159632906391665,
        cauchy.logpdf(&0.5, (3.5, 1.5)),
        epsilon = LOGPDF_EPSILON
    );
}

#[test]
pub fn test_dirichlet() {
    approx::assert_abs_diff_eq!(
        1.5040773967762744,
        dirichlet.logpdf(&vec![0.2, 0.3, 0.5], vec![1.0, 2.0, 3.0]),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        0.4054651081084364,
        dirichlet.logpdf(&vec![0.5, 0.5], vec![2.0, 2.0]),
        epsilon = LOGPDF_EPSILON
    );
    approx::assert_abs_diff_eq!(
        1.5609788413006192,
        dirichlet.logpdf(&vec![0.1, 0.6, 0.3], vec![0.5, 3.0, 1.5]),
        epsilon = LOGPDF_EPSILON
    );
}
