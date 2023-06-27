use std::rc::Rc;
use std::any::Any;
use rand::rngs::ThreadRng;
use nalgebra::DMatrix;
use gen_rs::{
    modeling::dists::{self, Distribution},
    GenerativeFunction, Trace, ChoiceHashMap, ChoiceBuffer
};
use super::types_2d::{Point,Bounds,uniform_2d};
use super::trace::PointedTrace;


pub struct PointedModel {
    pub obs_cov: DMatrix<f64>
}

impl GenerativeFunction for PointedModel {

    type X = Bounds;
    type T = Point;
    type U = PointedTrace;

    fn simulate(&self, rng: &mut ThreadRng, bounds: Self::X) -> Self::U {
        let latent = uniform_2d.random(rng, bounds);
        let obs = dists::mvnormal.random(rng, (&latent, &self.obs_cov));
        let mut constraints = ChoiceHashMap::new();
        constraints.set_value("latent", &Rc::new(latent));
        constraints.set_value("obs", &Rc::new(obs));
        PointedTrace::new(bounds, constraints, 0.)
    }

    fn generate(&self, rng: &mut ThreadRng, bounds: Self::X, constraints: impl ChoiceBuffer) -> Self::U {
        let mut weight = 0.;
        let mut choices = ChoiceHashMap::new();

        // manual latent branch
        let latent_choice: Rc<Point>;
        if constraints.has_value("latent") {
            latent_choice = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            weight += uniform_2d.logpdf(&latent_choice, bounds);
        } else {
            latent_choice = Rc::new(uniform_2d.random(rng, bounds));
        }
        choices.set_value("latent", &latent_choice);

        // manual obs branch
        let obs_choice: Rc<Point>;
        if constraints.has_value("obs") {
            obs_choice = (constraints.get_value("obs") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            weight += dists::mvnormal.logpdf(&obs_choice, (&latent_choice, &self.obs_cov));
        } else {
            obs_choice = Rc::new(dists::mvnormal.random(rng, (&latent_choice, &self.obs_cov)));
        }
        choices.set_value("obs", &obs_choice);

        PointedTrace::new(bounds, choices, weight)
    }

    fn propose(&self, _: &mut ThreadRng, _: Self::X) -> (ChoiceHashMap<Point>, f64) {
        panic!("not implemented")
    }

    fn assess(&self, _: &mut ThreadRng, _: Self::X, _: impl ChoiceBuffer) -> f64 {
        panic!("not implemented")
    }

    fn update(&self, trace: Rc<Self::U>, constraints: impl ChoiceBuffer) -> (Self::U, ChoiceHashMap<Point>) {
        let prev_choices = trace.get_choices() as ChoiceHashMap<Point>;
        let bounds = trace.get_args();
        let mut new_choices = ChoiceHashMap::<Point>::new();
        let mut discard = ChoiceHashMap::<Point>::new();

        let mut new_score = 0.;

        let mut latent_choice = prev_choices["latent"].clone();
        if constraints.has_value("latent") {
            discard.set_value("latent", &latent_choice);
            latent_choice = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            new_score = new_score - uniform_2d.logpdf(&prev_choices["latent"], *bounds)
        }
        new_score = new_score + uniform_2d.logpdf(&latent_choice, *bounds);
        new_choices.set_value("latent", &latent_choice);

        let mut obs_choice = prev_choices["obs"].clone();
        if constraints.has_value("obs") {
            discard.set_value("obs", &obs_choice);
            obs_choice = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            new_score = new_score - dists::mvnormal.logpdf(&prev_choices["obs"], (&latent_choice, &self.obs_cov));
        }
        new_score = new_score + dists::mvnormal.logpdf(&obs_choice, (&latent_choice, &self.obs_cov));
        new_choices.set_value("obs", &obs_choice);

        let new_trace = PointedTrace::new(
            trace.get_args().clone(),
            new_choices,
            new_score
        );
        (new_trace, discard)
    }
}