use std::f32::consts::PI;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;

use crate::types_2d;


// Distributions

pub trait Distribution<U,T> {
    fn logpdf(&self, params: &U, x: &T) -> f32;
    fn random(&self, rng: &mut ThreadRng, params: &U) -> T;
}

pub struct Uniform2D { }
pub const uniform_2d: Uniform2D = Uniform2D { };

impl Distribution<types_2d::Bounds,types_2d::Point> for Uniform2D {
    fn logpdf(&self, b: &types_2d::Bounds, _: &types_2d::Point) -> f32 {
        assert!(b.xmax > b.xmin);
        assert!(b.ymax > b.ymin);
        -((b.xmax - b.xmin) as f32 * (b.ymax - b.ymin) as f32).ln()
    }

    fn random(&self, rng: &mut ThreadRng, b: &types_2d::Bounds) -> types_2d::Point {
        types_2d::Point {
            x: rng.sample(Uniform::new(b.xmin,b.xmax)),
            y: rng.sample(Uniform::new(b.xmin,b.ymax)),
        }
    }
}

pub struct Normal { }
pub const normal: Normal = Normal { };

impl Distribution<(f32,f32),f32> for Normal {
    fn logpdf(&self, params: &(f32,f32), x: &f32) -> f32 {
        let (mu, std) = params;
        let z = (x - mu) / std;
        -(z.abs().powf(2.) + (2.*PI))/2. - std.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: &(f32,f32)) -> f32 {
        let u: f32 = (rng.sample(Uniform::new(0.,1.))) * 2. - 1.;
        let v: f32 = (rng.sample(Uniform::new(0.,1.))) * 2. - 1.;
        let r: f32 = u * u + v * v;
        if r == 0. || r > 1. { return self.random(rng, params); }
        let c = f32::powf(-2. * r.ln() / r, 0.5);
        return u * c;
    }
}

pub struct Categorical { }
pub const categorical: Categorical = Categorical { };

impl Distribution<Vec<f32>,usize> for Categorical {
    fn logpdf(&self, params: &Vec<f32>, x: &usize) -> f32 {
        0.
    }

    fn random(&self, rng: &mut ThreadRng, params: &Vec<f32>) -> usize {
        1
    }
}