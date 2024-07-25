use std::sync::Weak;
use nalgebra::DMatrix;
use modppl::{Distribution, mvnormal, GenFn, Trace, ArgDiff};
use super::model::PointedBuffer;
use super::types_2d::{Point,Bounds};
use rand::rngs::ThreadRng;


pub struct DriftProposal {
    pub drift_cov: DMatrix<f64>
}

pub type DriftProposalArgs = (Weak<Trace<Bounds,PointedBuffer,Point>>,());

impl GenFn<DriftProposalArgs,PointedBuffer,()> for DriftProposal {

    fn simulate(&self, args: DriftProposalArgs) -> Trace<DriftProposalArgs,PointedBuffer,()> {
        let mut rng = ThreadRng::default();
        let prev_trace = args.0.upgrade().unwrap();
        let mut choices = (None, prev_trace.data.1.clone());

        let new_latent = mvnormal.random(&mut rng, (prev_trace.data.0.clone().unwrap(), self.drift_cov.clone()));
        choices.0 = Some(new_latent);
        let logp = mvnormal.logpdf(&choices.0.clone().unwrap(), (prev_trace.data.0.clone().unwrap(), self.drift_cov.clone()));

        Trace::new(args, choices, (), logp)
    }

    fn generate(&self, args: DriftProposalArgs, constraints: PointedBuffer) -> (Trace<DriftProposalArgs,PointedBuffer,()>, f64) {
        let prev_trace = args.0.upgrade().unwrap();
        let mut choices = (None, prev_trace.data.1.clone());

        let new_latent: Point;
        let logp: f64;
        let mut weight = 0.;
        match constraints.0 {
            Some(latent_constraint) => {
                new_latent = latent_constraint;
                logp = mvnormal.logpdf(&new_latent, (prev_trace.data.0.clone().unwrap(), self.drift_cov.clone()));
                weight = logp;
            }
            None => {
                let mut rng = ThreadRng::default();
                new_latent = mvnormal.random(&mut rng, (prev_trace.data.0.clone().unwrap(), self.drift_cov.clone()));
                logp = mvnormal.logpdf(&new_latent, (prev_trace.data.0.clone().unwrap(), self.drift_cov.clone()));
            }
        }
        choices.0 = Some(new_latent);

        (Trace::new(args, choices, (), logp), weight)
    }

    fn update(&self, _: Trace<DriftProposalArgs,PointedBuffer,()>, _: DriftProposalArgs, _: ArgDiff, _: PointedBuffer) -> (Trace<DriftProposalArgs,PointedBuffer,()>, PointedBuffer, f64) {
        panic!("not implemented")
    }
}