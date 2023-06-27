use std::rc::Rc;
use gen_rs::{
    ChoiceBuffer, Trace, ChoiceHashMap
};
use super::types_2d::{Point,Bounds};


// a PointedTrace is a simple execution trace of a model
// whose random variables are well-represented by a (flat)
// hashmap from addresses to values.

// This mostly covers models that don't utilize splicing,
// including recursive filters.
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
}

impl Trace for PointedTrace {

    type X = Bounds;
    type T = Point;

    fn get_args(&self) -> &Self::X {
        &self.args
    }

    fn get_choices(&self) -> ChoiceHashMap<Point> {
        let mut choices = ChoiceHashMap::new();
        choices.set_value("latent", &Rc::clone(&self.choices["latent"]));
        choices.set_value("obs", &Rc::clone(&self.choices["obs"]));
        choices
    }

    fn get_retval(&self) -> &Self::T {
        &self.choices["obs"]
    }

    fn get_score(&self) -> f64 {
        self.score
    }

}