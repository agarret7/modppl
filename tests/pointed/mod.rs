pub mod trace;

use std::rc::Rc;
use std::any::Any;
use rand::rngs::ThreadRng;
use genark::{
    types_2d,
    modeling::dists::{self, Distribution},
    GenerativeFunction, ChoiceHashMap, ChoiceBuffer
};
pub use trace::PointedTrace;


pub struct PointedModel {
    pub obs_std: f32
}

impl GenerativeFunction for PointedModel {
    type X = types_2d::Bounds;
    type T = types_2d::Point;
    type U = PointedTrace;

    fn simulate(&self, rng: &mut ThreadRng, bounds: Rc<types_2d::Bounds>) -> Self::U {
        let latent = dists::uniform_2d.random(rng, &bounds);
        let obs = types_2d::Point {
            x: dists::normal.random(rng, &(latent.x, self.obs_std)),
            y: dists::normal.random(rng, &(latent.y, self.obs_std))
        };
        let mut constraints = ChoiceHashMap::new();
        constraints.set_value("latent", &Rc::new(latent));
        constraints.set_value("obs", &Rc::new(obs));
        PointedTrace::new(bounds, constraints, 0.)
    }

    fn generate(&self, rng: &mut ThreadRng, bounds: Rc<Self::X>, choices: impl ChoiceBuffer) -> Self::U {
        let mut weight = 0.;

        let latent_choice: Rc<types_2d::Point>;

        // manual latent branch
        if choices.has_value("latent") {
            latent_choice = Rc::clone((choices.get_value("latent") as &dyn Any).downcast_ref::<Rc<types_2d::Point>>().unwrap());
            weight += dists::uniform_2d.logpdf(&latent_choice, &bounds);
        } else {
            latent_choice = Rc::new(dists::uniform_2d.random(rng, &bounds));
        }

        // manual obs branch
        let obs_choice: Rc<types_2d::Point>;

        if choices.has_value("obs") {
            obs_choice = Rc::clone((choices.get_value("obs") as &dyn Any).downcast_ref::<Rc<types_2d::Point>>().unwrap());
            weight += dists::normal.logpdf(&obs_choice.x, &(latent_choice.x, self.obs_std))
                + dists::normal.logpdf(&obs_choice.y, &(latent_choice.y, self.obs_std));

        } else {
            obs_choice = Rc::new(types_2d::Point {
                x: dists::normal.random(rng, &(latent_choice.x, self.obs_std)),
                y: dists::normal.random(rng, &(latent_choice.y, self.obs_std))
            });
        }

        let mut constraints = ChoiceHashMap::new();
        constraints.set_value("latent", &latent_choice);
        constraints.set_value("obs", &obs_choice);

        PointedTrace::new(bounds, constraints, weight)
    }

    fn update(&self, trace: PointedTrace, fwd_choices: impl ChoiceBuffer) -> Self::U {
        trace
    }

    fn revert(&self, trace: PointedTrace, bwd_choices: impl ChoiceBuffer) -> Self::U {
        trace
    }
}