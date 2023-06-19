use std::rc::Rc;
use genark::{
    types_2d,
    ChoiceBuffer, Trace, ChoiceHashMap, GenerativeFunction
};

pub struct PointedTrace {
    args: Rc<types_2d::Bounds>,
    choices: ChoiceHashMap<types_2d::Point>,
    score: f32
}

impl PointedTrace {
    pub fn new(
        args: Rc<types_2d::Bounds>,
        choices: ChoiceHashMap<types_2d::Point>,
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

    type X = types_2d::Bounds;
    type T = types_2d::Point;

    fn get_args(&self) -> &Rc<Self::X> {
        &self.args
    }

    fn get_choices(&self) -> ChoiceHashMap<types_2d::Point> {
        let mut choices = ChoiceHashMap::new();
        choices.set_value("latent", &Rc::clone(&self.choices["latent"]));
        choices.set_value("obs", &Rc::clone(&self.choices["obs"]));
        choices
    }

    fn get_retval(&self) -> &Rc<Self::T> {
        &self.choices["obs"]
    }

    fn get_score(&self) -> f32 {
        self.score
    }

}