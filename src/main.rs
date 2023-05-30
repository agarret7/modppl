// pub mod modeling;
// pub mod inference;

use std::f64::consts::PI;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

trait Distribution<U,T> {
    fn logpdf(&self, params: &U, x: &T) -> f64;
    fn random(&self, rng: &mut ThreadRng, params: &U) -> T;
}

struct Bounds { xmin: i32, xmax: i32, ymin: i32, ymax: i32 }

struct Uniform2D { }
const uniform_2d: Uniform2D = Uniform2D { };


impl Distribution<Bounds,Point> for Uniform2D {
    fn logpdf(&self, b: &Bounds, _: &Point) -> f64 {
        // todo make some simple boundary checks
        -((b.xmax - b.xmin) as f64 * (b.ymax - b.ymin) as f64).ln()
    }

    fn random(&self, rng: &mut ThreadRng, b: &Bounds) -> Point {
        Point {
            x: rng.sample(Uniform::new(b.xmin,b.xmax)),
            y: rng.sample(Uniform::new(b.xmin,b.ymax)),
        }
    }
}

struct Normal { }
const normal: Normal = Normal { };

impl Distribution<(f64,f64),f64> for Normal {
    fn logpdf(&self, params: &(f64,f64), x: &f64) -> f64 {
        let (mu, std) = params;
        let z = (x - mu) / std;
        -(z.abs().powf(2.) + (2.*PI))/2. - std.ln()
    }

    fn random(&self, rng: &mut ThreadRng, params: &(f64,f64)) -> f64 {
        let u: f64 = (rng.sample(Uniform::new(0.,1.))) * 2. - 1.;
        let v: f64 = (rng.sample(Uniform::new(0.,1.))) * 2. - 1.;
        let r: f64 = u * u + v * v;
        if r == 0. || r > 1. { return self.random(rng, params); }
        let c = f64::powf(-2. * r.ln() / r, 0.5);
        return u * c;
    }
}

struct Categorical { }
const categorical: Categorical = Categorical { };

impl Distribution<Vec<f64>,usize> for Categorical {
    fn logpdf(&self, params: &Vec<f64>, x: &usize) -> f64 {
        0.
    }

    fn random(&self, rng: &mut ThreadRng, params: &Vec<f64>) -> usize {
        1
    }
}

fn simulate_loop(b: &Bounds, T: u32) -> Vec<Point> {
    let xrange = (b.xmax - b.xmin) as f64;
    let yrange = (b.ymax - b.ymin) as f64;
    let cx = xrange / 2.;
    let cy = yrange / 2.;
    let r = std::cmp::max(b.xmax - b.xmin, b.ymax - b.ymin) as f64 / 1.5;
    let mut observations = vec![];
    for t in 1..T {
        let u = (t as f64) / (10.*PI);
        let t = (t as f64) / (2.*PI);
        let obs = Point { x: (cx + t.cos() + u.cos()).round() as i32, y: (cy + t.sin() + u.sin()).round() as i32};
        observations.push(obs);
    }
    observations
}

fn initialize_particles(rng: &mut ThreadRng, N: usize, b: &Bounds) -> Vec<(Point,f64)> {
    let mut particles = vec![];
    for _ in 0..N {
        let particle = uniform_2d.random(rng, b);
        let weight = uniform_2d.logpdf(b, &particle);
        particles.push((particle, weight));
    }
    particles
}

fn importance_resample(particles: &Vec<(Point,f64)>, n: u32, obs: &Point) -> Vec<(Point,f64)> { 
    let new_particles = vec![];
    // generate random indices proportional to categorical
    for p in particles {
    }
    new_particles
}

fn particle_updater<'a>(rng: &'a mut ThreadRng, particles: &'a mut Vec<(Point,f64)>, new_obs: &Point) {
    for particle in particles.iter_mut() {
        let (p, w) = particle;
        let x_std = 10.;
        let y_std = 10.;
        let dx = normal.random(rng, &(0.,x_std)) as i32;
        let dy = normal.random(rng, &(0.,y_std)) as i32;
        let proposed_p = Point { x: p.x + dx, y: p.y + dy };
        let inc = normal.logpdf(&(proposed_p.x as f64, x_std), &(new_obs.x as f64))
                     + normal.logpdf(&(proposed_p.y as f64, y_std), &(new_obs.y as f64));
        *particle = (proposed_p, *w + inc);
    }
}

fn main() {
    let T = 100;
    let init_N = 1;
    let b = Bounds { xmin: 0, xmax: 600, ymin: 0, ymax: 400 };
    let observations = simulate_loop(&b, T);
    let mut rng = ThreadRng::default();
    let mut particles = initialize_particles(&mut rng, init_N, &b);
    // todo: resample according to importance weights
    for obs in observations {
        particle_updater(&mut rng, &mut particles, &obs);
    }
}