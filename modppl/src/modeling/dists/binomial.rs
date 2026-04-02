use rand::rngs::ThreadRng;
use super::Distribution;
use rand_distr::{
    Distribution as _,
    Binomial as BinomialSampler
};


/// Binomial distribution type
pub struct Binomial { }

/// Instantiation of the binomial distribution
pub const binomial: Binomial = Binomial { };

impl Distribution<i64,(i64,f64)> for Binomial {
    fn logpdf(&self, k: &i64, params: (i64,f64)) -> f64 {
        let (n, p) = params;
        let log_binom = (0..*k).map(|i| ((n - i) as f64).ln() - ((i + 1) as f64).ln()).sum::<f64>();
        log_binom + (*k as f64)*p.ln() + ((n - k) as f64)*(1. - p).ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (i64,f64)) -> i64 {
        let (n, p) = params;
        let binom_sampler = BinomialSampler::new(n as u64, p).ok().unwrap();
        binom_sampler.sample(rng) as i64
    }
}
