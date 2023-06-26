use std::f32::consts::PI;
use rand::{self,Rng, rngs::ThreadRng};

use crate::types_2d;


// Distributions

pub fn u01(rng: &mut ThreadRng) -> f32 {
    rng.sample(rand::distributions::Uniform::new(0., 1.))
}

pub trait Distribution<T,U> {
    fn logpdf(&self, x: &T, params: &U) -> f32;
    fn random(&self, rng: &mut ThreadRng, params: &U) -> T;
}

pub struct Bernoulli { }
pub const bernoulli: Bernoulli = Bernoulli { };

impl Distribution<bool,f32> for Bernoulli {
    fn logpdf(&self, a: &bool, p: &f32) -> f32 {
        (if *a { *p } else { 1. - *p }).ln()
    }

    fn random(&self, rng: &mut ThreadRng, p: &f32) -> bool {
        *p > u01(rng)
    }
}


pub struct Normal { }
pub const normal: Normal = Normal { };

impl Distribution<f32,(f32,f32)> for Normal {
    fn logpdf(&self, x: &f32, params: &(f32,f32)) -> f32 {
        let (mu, std) = params;
        let z = (x - mu) / std;
        -(z.abs().powf(2.) + (2.*PI).ln())/2. - std.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: &(f32,f32)) -> f32 {
        let (mu, std) = params;
        let u: f32 = u01(rng) * 2. - 1.;
        let v: f32 = u01(rng) * 2. - 1.;
        let r: f32 = u * u + v * v;
        if r == 0. || r > 1. { return self.random(rng, params); }
        let c = (-2. * r.ln() / r).sqrt();
        return u * c * std + mu;
    }
}

pub struct Uniform2D { }
pub const uniform_2d: Uniform2D = Uniform2D { };

impl Distribution<types_2d::Point,types_2d::Bounds> for Uniform2D {
    fn logpdf(&self, p: &types_2d::Point, b: &types_2d::Bounds) -> f32 {
        return if b.xmin <= p.x && p.x <= b.xmax && b.ymin <= p.y && p.y <= b.ymax {
            -((b.xmax - b.xmin) as f32 * (b.ymax - b.ymin) as f32).ln()
        } else {
            -f32::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, b: &types_2d::Bounds) -> types_2d::Point {
        assert!(b.xmax > b.xmin);
        assert!(b.ymax > b.ymin);
        types_2d::Point {
            x: u01(rng)*(b.xmax - b.xmin) + b.xmin,
            y: u01(rng)*(b.ymax - b.ymin) + b.ymin
        }
    }
}

pub struct Categorical { }
pub const categorical: Categorical = Categorical { };

impl Distribution<usize,Vec<f32>> for Categorical {
    fn logpdf(&self, x: &usize, probs: &Vec<f32>) -> f32 {
        return if (*x as i32) > 0 && (*x as i32) <= probs.len() as i32 {
            probs[*x].ln()
        } else {
            -f32::INFINITY
        }
    }

    fn random(&self, rng: &mut ThreadRng, probs: &Vec<f32>) -> usize {
        let u = u01(rng);
        let mut t = 0.;
        let mut x: usize = 0;
        while t < u {
            t += probs[x];
            x += 1;
        }
        return x - 1;
    }
}