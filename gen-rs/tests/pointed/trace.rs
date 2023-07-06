use std::rc::Rc;
use gen_rs::{
    ChoiceBuffer, Trace, ChoiceHashMap
};
use super::types_2d::{Point,Bounds};


pub struct PointedTrace {
    args: Bounds,
    choices: ChoiceHashMap<Point>,
    score: f64
}

impl PointedTrace {
    pub fn new(
        args: Bounds,
        choices: ChoiceHashMap<Point>,
        score: f64
    ) -> PointedTrace {
        PointedTrace {
            args: args,
            choices: choices,
            score: score
        }
    }

    pub fn set_latent(&mut self, value: &Rc<Point>) {
        self.choices.set_value("latent", value);
    }

    pub fn set_obs(&mut self, value: &Rc<Point>) {
        self.choices.set_value("obs", value);
    }
}

impl Trace for PointedTrace {

    type X = Bounds;
    type T = Point;

    fn get_args(&self) -> &Self::X {
        &self.args
    }

    fn get_choices(&self) -> ChoiceHashMap<Point> {
        self.choices.clone()
    }

    fn get_retval(&self) -> &Self::T {
        &self.choices["obs"]
    }

    fn get_score(&self) -> f64 {
        self.score
    }

    fn set_score(&mut self, new_score: f64) {
        self.score = new_score;
    }

}