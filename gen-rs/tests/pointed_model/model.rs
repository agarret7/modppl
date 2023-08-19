use nalgebra::DMatrix;
use gen_rs::{GLOBAL_RNG, Distribution, mvnormal, Trace, GenFn, GfDiff};
use super::types_2d::{Point,Bounds,uniform_2d};


pub struct PointedModel {
    pub obs_cov: DMatrix<f64>
}

pub type PointedBuffer = (Option<Point>,Option<Point>);
pub type PointedTrace = Trace<Bounds,PointedBuffer,Point>;

impl GenFn<Bounds,PointedBuffer,Point> for PointedModel {

    fn simulate(&self, bounds: Bounds) -> PointedTrace {
        GLOBAL_RNG.with_borrow_mut(|rng| {
            let mut logp = 0.;
            let latent = uniform_2d.random(rng, bounds);
            logp += uniform_2d.logpdf(&latent, bounds);
            let obs = mvnormal.random(rng, (latent.clone(), self.obs_cov.clone()));
            logp += mvnormal.logpdf(&obs, (obs.clone(), self.obs_cov.clone()));
            PointedTrace::new(bounds, (Some(latent), Some(obs.clone())), obs, logp)
        })
    }

    fn generate(&self, bounds: Bounds, constraints: PointedBuffer) -> (PointedTrace, f64) {
        let mut logp = 0.;
        let mut weight = 0.;
        let mut choices = (None, None);

        // manual latent branch
        let latent_choice = match constraints.0 {
            Some(constrained_latent) => {
                let new_weight = uniform_2d.logpdf(&constrained_latent, bounds);
                weight += new_weight;
                logp += new_weight;
                constrained_latent
            }
            None => {
                GLOBAL_RNG.with_borrow_mut(|rng| {
                    let latent_choice = uniform_2d.random(rng, bounds);
                    let new_weight = uniform_2d.logpdf(&latent_choice, bounds);
                    logp += new_weight;
                    latent_choice
                })
            }
        };
        choices.0 = Some(latent_choice.clone());

        // manual obs branch
        let obs_choice = match constraints.1 {
            Some(constrained_obs) => {
                let new_weight = mvnormal.logpdf(&constrained_obs, (latent_choice, self.obs_cov.clone()));
                weight += new_weight;
                logp += new_weight;
                constrained_obs
            }
            None => {
                GLOBAL_RNG.with_borrow_mut(|rng| {
                    let obs_choice = mvnormal.random(rng, (latent_choice.clone(), self.obs_cov.clone()));
                    let new_weight = mvnormal.logpdf(&obs_choice, (latent_choice, self.obs_cov.clone()));
                    logp += new_weight;
                    obs_choice
                })
            }
        };
        choices.1 = Some(obs_choice.clone());

        (PointedTrace::new(bounds, choices, obs_choice, logp), weight)
    }

    fn update(&self, trace: PointedTrace, args: Bounds, diff: GfDiff, constraints: PointedBuffer) -> (PointedTrace, PointedBuffer, f64) {
        match diff {
            GfDiff::NoChange => {
                let prev_choices = trace.data;
                let bounds = trace.args;
                let mut discard = (None, None);

                let mut new_logp = trace.logp;
                let mut visited_obs = false;

                let mut latent_choice = prev_choices.0.clone();
                if let Some(latent_constraint) = constraints.0 {
                    discard.0 = latent_choice;
                    latent_choice = Some(latent_constraint.clone());
                    new_logp -= uniform_2d.logpdf(&prev_choices.0.clone().unwrap(), bounds);
                    new_logp += uniform_2d.logpdf(&latent_constraint, bounds);

                    visited_obs = true;
                    new_logp -= mvnormal.logpdf(&prev_choices.1.clone().unwrap(), (prev_choices.0.clone().unwrap(), self.obs_cov.clone()));
                }

                let mut obs_choice = prev_choices.1.clone();
                if let Some(obs_constraint) = constraints.1 {
                    discard.1 = obs_choice;
                    obs_choice = Some(obs_constraint);
                    if !visited_obs {
                        new_logp -= mvnormal.logpdf(&prev_choices.1.unwrap(), (prev_choices.0.clone().unwrap(), self.obs_cov.clone()));
                    }
                    new_logp += mvnormal.logpdf(&obs_choice.clone().unwrap(), (latent_choice.clone().unwrap(), self.obs_cov.clone()));
                } else if visited_obs {
                    new_logp += mvnormal.logpdf(&obs_choice.clone().unwrap(), (latent_choice.clone().unwrap(), self.obs_cov.clone()));
                }

                (PointedTrace::new(args, (latent_choice, obs_choice.clone()), obs_choice.unwrap(), new_logp), discard, new_logp - trace.logp)
            },
            _ => { panic!("Can't handle GF change type: {:?}", diff) },
        }
    }

}