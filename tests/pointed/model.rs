use std::rc::Rc;
use std::any::Any;
use rand::rngs::ThreadRng;
use genark::{
    types_2d::{Bounds,Point},
    modeling::dists::{self, Distribution},
    GenerativeFunction, Trace, ChoiceHashMap, ChoiceBuffer
};
pub use super::trace::PointedTrace;


pub struct PointedModel {
    pub obs_std: f32
}

impl GenerativeFunction for PointedModel {

    type X = Bounds;
    type T = Point;
    type U = PointedTrace;

    fn simulate(&self, rng: &mut ThreadRng, bounds: Rc<Self::X>) -> Self::U {
        let latent = dists::uniform_2d.random(rng, &bounds);
        let obs = Point {
            x: dists::normal.random(rng, &(latent.x, self.obs_std)),
            y: dists::normal.random(rng, &(latent.y, self.obs_std))
        };
        let mut constraints = ChoiceHashMap::new();
        constraints.set_value("latent", &Rc::new(latent));
        constraints.set_value("obs", &Rc::new(obs));
        PointedTrace::new(bounds, constraints, 0.)
    }

    fn generate(&self, rng: &mut ThreadRng, bounds: Rc<Self::X>, constraints: impl ChoiceBuffer) -> Self::U {
        let mut weight = 0.;
        let mut choices = ChoiceHashMap::new();

        // manual latent branch
        let latent_choice: Rc<Point>;
        if constraints.has_value("latent") {
            latent_choice = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            weight += dists::uniform_2d.logpdf(&latent_choice, &bounds);
        } else {
            latent_choice = Rc::new(dists::uniform_2d.random(rng, &bounds));
        }
        choices.set_value("latent", &latent_choice);

        // manual obs branch
        let obs_choice: Rc<Point>;
        if constraints.has_value("obs") {
            obs_choice = (constraints.get_value("obs") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            weight += dists::normal.logpdf(&obs_choice.x, &(latent_choice.x, self.obs_std))
                + dists::normal.logpdf(&obs_choice.y, &(latent_choice.y, self.obs_std));
        } else {
            obs_choice = Rc::new(Point {
                x: dists::normal.random(rng, &(latent_choice.x, self.obs_std)),
                y: dists::normal.random(rng, &(latent_choice.y, self.obs_std))
            });
        }
        choices.set_value("obs", &obs_choice);

        PointedTrace::new(bounds, choices, weight)
    }

    fn propose(&self, _: &mut ThreadRng, _: Rc<Self::X>) -> (ChoiceHashMap<Point>, f32) {
        // this is wrong, but we don't call propose on this GF.
        (ChoiceHashMap::new(), 0.)
    }

    fn assess(&self, _: &mut ThreadRng, _: Rc<Self::X>, _: impl ChoiceBuffer) -> f32 {
        // this is wrong, but we don't call assess on this GF.
        return 0.
    }

    fn update(&self, trace: Rc<Self::U>, constraints: impl ChoiceBuffer) -> (Self::U, ChoiceHashMap<Point>) {
        // this is wrong
        let old_choices = trace.get_choices() as ChoiceHashMap<Point>;
        let mut new_choices = ChoiceHashMap::<Point>::new();
        let mut discard = ChoiceHashMap::<Point>::new();

        let mut latent_choice = (old_choices.get_value("latent") as &dyn Any)
            .downcast_ref::<Rc<Point>>()
            .unwrap()
            .clone();
        if constraints.has_value("latent") {
            discard.set_value("latent", &latent_choice);
            latent_choice = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
        }
        new_choices.set_value("latent", &latent_choice);

        let obs_choice: Rc<Point>;
        if constraints.has_value("obs") {

        } else {

        }

        let new_trace = PointedTrace::new(
            trace.get_args(),
            trace.get_choices(),
            trace.get_score()
        );
        (new_trace, discard)
    }
}