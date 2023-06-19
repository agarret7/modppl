use std::collections::HashMap;
use rand::rngs::ThreadRng;
use math::stats;
use approx;
use genark::{
    types_2d,
    modeling::dists::{
        Distribution,
        normal,
        uniform_2d,
        categorical
    }
};

#[test]
fn test_normal() {
    let mut rng = ThreadRng::default();

    let samples = (0..50000).map(|_| normal.random(&mut rng, &(1.64, 0.025))).collect::<Vec<f32>>();
    let empirical_mu = stats::mean(samples.iter());
    let empirical_std = stats::standard_deviation(samples.iter(), 1);
    approx::assert_abs_diff_eq!(empirical_mu, 1.64, epsilon = 0.02);
    approx::assert_abs_diff_eq!(empirical_std, 0.025, epsilon = 0.02);

    let x = 1.4;
    let mu = 0.9;
    let std = 0.5;
    let logp = normal.logpdf(&x, &(mu, std));
    approx::assert_abs_diff_eq!(logp, -0.7257913507400731, epsilon = f32::EPSILON);

    let x = 2.8;
    let mu = 1.8;
    let std = 1.;
    let logp = normal.logpdf(&x, &(mu, std));
    approx::assert_abs_diff_eq!(logp, -1.4189385332046727, epsilon = f32::EPSILON);

    let x = -3.14;
    let mu = 8.;
    let std = 20.;
    let logp = normal.logpdf(&x, &(mu, std));
    approx::assert_abs_diff_eq!(logp, -4.069795370834944, epsilon = f32::EPSILON);
}

#[test]
fn test_uniform2d() {
    let mut rng = ThreadRng::default();
    let bounds = types_2d::Bounds { xmin: 0., xmax: 2.5, ymin: -1., ymax: 0.25 };

    let samples = (0..50000).map(|_| uniform_2d.random(&mut rng, &bounds)).collect::<Vec<types_2d::Point>>();
    for sample in samples {
        assert!(sample.x >= bounds.xmin);
        assert!(sample.x <= bounds.xmax);
        assert!(sample.y >= bounds.ymin);
        assert!(sample.y <= bounds.ymax);
        approx::assert_abs_diff_eq!(uniform_2d.logpdf(&sample, &bounds), -1.1394343, epsilon=f32::EPSILON);
    }

    assert_eq!(uniform_2d.logpdf(&types_2d::Point { x: -1., y: 0.}, &bounds), -f32::INFINITY);
}

#[test]
fn test_categorical() {
    let mut rng = ThreadRng::default();
    let labels = vec!["a", "b", "c", "d", "e", "f"];
    let probs = vec![0.1, 0.3, 0.2, 0.1, 0.05, 0.25];
    let num_samples = 50000;
    let sample_indices = (0..num_samples).map(|_| categorical.random(&mut rng, &probs)).collect::<Vec<usize>>();

    let samples = sample_indices.iter().map(|idx| labels[*idx]).collect::<Vec<&str>>();

    let mut count = HashMap::new();

    for item in samples.iter() {
        *count.entry(item).or_insert(0) += 1;
    }
    for (i, gt_freq) in (0..6).zip(probs.iter()) {
        let freq = count[&labels[i]] as f32 / num_samples as f32;
        approx::assert_abs_diff_eq!(freq, gt_freq, epsilon = 0.01);
    }
}