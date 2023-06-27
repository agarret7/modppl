use rand::rngs::ThreadRng;
use super::{Distribution,u01};
use std::f64::consts::PI;


pub struct Normal { }
pub const normal: Normal = Normal { };

impl Distribution<f64,(f64,f64)> for Normal {
    fn logpdf(&self, x: &f64, params: (f64,f64)) -> f64 {
        let (mu, std) = params;
        let z = (x - mu) / std;
        -(z.abs().powf(2.) + (2.*PI).ln())/2. - std.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: (f64,f64)) -> f64 {
        let (mu, std) = params;
        let u: f64 = u01(rng) * 2. - 1.;
        let v: f64 = u01(rng) * 2. - 1.;
        let r: f64 = u * u + v * v;
        if r == 0. || r > 1. { return self.random(rng, params); }
        let c = (-2. * r.ln() / r).sqrt();
        return u * c * std + mu;
    }
}