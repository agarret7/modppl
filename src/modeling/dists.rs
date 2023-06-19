use std::f32::consts::PI;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;

use crate::types_2d;


// Distributions

pub trait Distribution<T,U> {
    fn logpdf(&self, x: &T, params: &U) -> f32;
    fn random(&self, rng: &mut ThreadRng, params: &U) -> T;
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
        let u: f32 = rng.sample(Uniform::new(0.,1.)) * 2. - 1.;
        let v: f32 = rng.sample(Uniform::new(0.,1.)) * 2. - 1.;
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
            x: rng.sample(Uniform::new(b.xmin,b.xmax)),
            y: rng.sample(Uniform::new(b.xmin,b.ymax)),
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
        let u = rng.sample(Uniform::new(1e-12, 1.));
        let mut t = 0.;
        let mut x: usize = 0;
        while t < u {
            t += probs[x];
            x += 1;
        }
        return x - 1;
    }
}