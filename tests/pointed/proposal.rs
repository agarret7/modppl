use std::rc::Rc;
use std::any::Any;
use rand::rngs::ThreadRng;
use genark::{
    types_2d::{Bounds,Point},
    modeling::dists::{self, Distribution},
    GenerativeFunction, Trace, ChoiceHashMap, ChoiceBuffer
};
pub use super::trace::PointedTrace;


pub struct DriftProposal {
    pub drift_std: f32
}

impl GenerativeFunction for DriftProposal {

    type X = (Rc<Self::U>, Rc<Bounds>);
    type T = Point;
    type U = PointedTrace;

    fn simulate(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> Self::U {
        let (prev_choices, bounds) = (args.0.get_choices() as ChoiceHashMap<Point>, args.1.clone());
        let mut choices = ChoiceHashMap::new();

        let new_latent = Rc::new(Point {
            x: dists::normal.random(rng, &(prev_choices["latent"].x, self.drift_std)),
            y: dists::normal.random(rng, &(prev_choices["latent"].y, self.drift_std))
        });
        choices.set_value("latent", &new_latent);

        PointedTrace::new(bounds, choices, 0.)
    }

    fn generate(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> Self::U {
        let (prev_choices, bounds) = (args.0.get_choices() as ChoiceHashMap<Point>, args.1.clone());
        let mut choices = ChoiceHashMap::new();
        let prev_obs = (prev_choices.get_value("obs") as &dyn Any)
            .downcast_ref::<Rc<Point>>()
            .unwrap();
        choices.set_value("obs", prev_obs);

        let new_latent: Rc<Point>;
        let weight;
        if constraints.has_value("latent") {
            new_latent = (constraints.get_value("latent") as &dyn Any)
                .downcast_ref::<Rc<Point>>()
                .unwrap()
                .clone();
            weight = dists::normal.logpdf(&prev_choices["latent"].x, &(new_latent.x, self.drift_std))
                + dists::normal.logpdf(&prev_choices["latent"].y, &(new_latent.y, self.drift_std));
        } else {
            new_latent = Rc::new(Point {
                x: dists::normal.random(rng, &(prev_choices["latent"].x, self.drift_std)),
                y: dists::normal.random(rng, &(prev_choices["latent"].y, self.drift_std))
            });
            weight = 0.
        }
        choices.set_value("latent", &new_latent);

        PointedTrace::new(bounds, choices, weight)
    }

    fn propose(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> (ChoiceHashMap<Point>, f32) {
        let prev_latent = args.0.get_choices()["latent"].clone();
        let new_choices = self.simulate(rng, args).get_choices();
        let new_latent = new_choices["latent"].clone();
        let weight = dists::normal.logpdf(&prev_latent.x, &(new_latent.x, self.drift_std))
            + dists::normal.logpdf(&prev_latent.y, &(new_latent.y, self.drift_std));
        (new_choices, weight)
    }

    fn assess(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> f32 {
        self.generate(rng, args, constraints).get_score()
    }

    fn update(&self, trace: Rc<PointedTrace>, _: impl ChoiceBuffer) -> (Self::U, ChoiceHashMap<Point>) {
        // this is wrong, but we don't call update on this GF.
        let new_trace = PointedTrace::new(
            trace.get_args(),
            trace.get_choices(),
            trace.get_score()
        );
        (new_trace, ChoiceHashMap::new())
    }
}