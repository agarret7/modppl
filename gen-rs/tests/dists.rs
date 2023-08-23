use std::collections::HashMap;
use nalgebra::{dvector,dmatrix};

use rand::rngs::ThreadRng;
use statistical::{mean, variance, standard_deviation};
use approx;
use gen_rs::{Distribution, bernoulli, uniform, uniform_discrete, categorical, normal, mvnormal};

#[test]
fn test_bernoulli() {
    let mut rng = ThreadRng::default();

    let true_p = 0.11;
    assert_eq!(bernoulli.logpdf(&true, true_p), true_p.ln());
    assert_eq!(bernoulli.logpdf(&false, true_p), (1.-true_p).ln());
    let samples = (0..50000).map(|_| bernoulli.random(&mut rng, 0.11)).collect::<Vec<bool>>();

    let empirical_true = samples.iter().filter(|&&x| x).collect::<Vec<_>>().len();
    let empirical_false = samples.iter().filter(|&&x| !x).collect::<Vec<_>>().len();
    let empirical_freq = empirical_true as f64 / empirical_false as f64;
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
    assert_eq!(uniform.logpdf(&0.4, params), f64::NEG_INFINITY);
    let num_samples = 50000;
    let samples = (0..num_samples).map(|_| uniform.random(&mut rng, params)).collect::<Vec<f64>>();
    let num_bins = 100;
    let mut hist = vec![0; num_bins];

    let expected_samples_per_bin = num_samples / num_bins;
    for x in samples {
        let i = ((x - a) / (b - a) * num_bins as f64).trunc() as usize;
        hist[i] += 1;
    }
    for bin_count in hist {
        approx::assert_abs_diff_eq!(bin_count, expected_samples_per_bin, epsilon = 150);
    }

    // discrete
    let params = (8, 130);
    let (a, b) = params;
    let true_p = 1. / (b - a + 1) as f64;
    assert_eq!(uniform_discrete.logpdf(&9, params), true_p.ln());
    assert_eq!(uniform_discrete.logpdf(&130, params), true_p.ln());
    assert_eq!(uniform_discrete.logpdf(&140, params), f64::NEG_INFINITY);
    let num_samples = 50000;
    let samples = (0..num_samples).map(|_| uniform_discrete.random(&mut rng, params)).collect::<Vec<i64>>();
    let num_bins = 5;
    let mut hist = vec![0; num_bins];

    let expected_samples_per_bin = num_samples / num_bins;
    for x in samples {
        let i = ((x - a) as f64 / (b - a + 1) as f64 * num_bins as f64) as usize;
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
    let sample_indices = (0..num_samples).map(|_| categorical.random(&mut rng, probs.clone())).collect::<Vec<usize>>();

    let samples = sample_indices.iter().map(|idx| labels[*idx]).collect::<Vec<&str>>();

    let mut count = HashMap::new();

    for item in samples.iter() {
        *count.entry(item).or_insert(0) += 1;
    }
    for (i, gt_freq) in (0..6).zip(probs.iter()) {
        let freq = count[&labels[i]] as f64 / num_samples as f64;
        approx::assert_abs_diff_eq!(freq, gt_freq, epsilon = 0.01);
    }
}

#[test]
fn test_normal() {
    let mut rng = ThreadRng::default();

    let true_mu = 1.64;
    let true_std = 0.025;

    let samples = (0..50000).map(|_| normal.random(&mut rng, (true_mu, true_std))).collect::<Vec<f64>>();

    let empirical_mu = mean(&samples);
    let empirical_std = standard_deviation(&samples, None);
    dbg!(&empirical_mu);
    dbg!(&empirical_std);
    approx::assert_abs_diff_eq!(empirical_mu, true_mu, epsilon = 0.001);
    approx::assert_abs_diff_eq!(empirical_std, true_std, epsilon = 0.001);

    let x = 1.4;
    let mu = 0.9;
    let std = 0.5;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -0.7257913526447272, epsilon = f64::EPSILON);

    let x = 2.8;
    let mu = 1.8;
    let std = 1.;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -1.4189385332046727, epsilon = f64::EPSILON);

    let x = -3.14;
    let mu = 8.;
    let std = 20.;
    let logp = normal.logpdf(&x, (mu, std));
    approx::assert_abs_diff_eq!(logp, -4.069795306758664, epsilon = f64::EPSILON);
}

#[test]
fn test_mvnormal() {
    let mut rng = ThreadRng::default();

    let true_mu = dvector![-1.5, 3.2];
    let true_cov = dmatrix![1.,-3./5.;-3./5.,2.];
    let params = (true_mu.clone(), true_cov.clone());

    let samples = (0..50000)
        .map(|_| mvnormal.random(&mut rng, params.clone()).data.as_vec().to_vec())
        .collect::<Vec<Vec<f64>>>();
    let sample_xs = samples.iter().map(|p| p[0]).collect::<Vec<f64>>();
    let sample_ys = samples.iter().map(|p| p[1]).collect::<Vec<f64>>();
    let e_mu_x = mean(&sample_xs);
    let e_mu_y = mean(&sample_ys);
    let e_mu = dvector![e_mu_x, e_mu_y];
    approx::assert_abs_diff_eq!(e_mu, true_mu, epsilon = 0.05);
    let e_var_x = variance(&sample_xs, None);
    let e_var_y = variance(&sample_ys, None);
    let e_cov_xy = sample_xs.iter().zip(sample_ys)
        .map(|(x,y)| (x - true_mu[0])*(y - true_mu[1]))
        .sum::<f64>() / samples.len() as f64;
    let e_cov = dmatrix![e_var_x, e_cov_xy; e_cov_xy, e_var_y];
    approx::assert_abs_diff_eq!(e_cov, true_cov, epsilon = 0.05);

    let x = dvector![1.1, 5.8];
    let mu = dvector![1.3, 5.6];
    let cov = dmatrix![1., -0.81; -0.81, 2.5];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -2.1642100746383357, epsilon = f64::EPSILON);

    let x = dvector![30.1, -46.8];
    let mu = dvector![0., 6.];
    let cov = dmatrix![496., 0.13; 0.13, 500.];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -11.750458919763666, epsilon = f64::EPSILON);

    let x = dvector![1.2, 5.1, -7.8];
    let mu = dvector![1.4, 5.0, -7.4];
    let cov = dmatrix![1., 0.1, 0.9; 0.1, 1.3, 0.4; 0.9, 0.4, 1.75];
    let params = (mu, cov);
    let logp = mvnormal.logpdf(&x, params);
    approx::assert_abs_diff_eq!(logp, -2.873267436425841, epsilon = f64::EPSILON);
}