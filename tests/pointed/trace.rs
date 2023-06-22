use std::rc::Rc;
use genark::{
    types_2d::{Bounds,Point},
    ChoiceBuffer, Trace, ChoiceHashMap
};

pub struct PointedTrace {
    args: Rc<Bounds>,
    choices: ChoiceHashMap<Point>,
    score: f32
}

impl PointedTrace {
    pub fn new(
        args: Rc<Bounds>,
        choices: ChoiceHashMap<Point>,
        score: f32
    ) -> PointedTrace {
        PointedTrace {
            args: args,
            choices: choices,
            score: score
        }
    }
}

impl Trace for PointedTrace {

    type X = Bounds;
    type T = Point;

    fn get_args(&self) -> Rc<Self::X> {
        self.args.clone()
    }

    fn get_choices(&self) -> ChoiceHashMap<Point> {
        let mut choices = ChoiceHashMap::new();
        choices.set_value("latent", &Rc::clone(&self.choices["latent"]));
        choices.set_value("obs", &Rc::clone(&self.choices["obs"]));
        choices
    }

    fn get_retval(&self) -> Rc<Self::T> {
        self.choices["obs"].clone()
    }

    fn get_score(&self) -> f32 {
        self.score
    }

}