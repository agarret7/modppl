use rand::rngs::ThreadRng;
use std::rc::Rc;

pub trait ChoiceBuffer {
    type K;
    type V;

    fn has_value(&self, k: Self::K) -> bool;
    fn get_value(&self, k: Self::K) -> &Rc<Self::V>;
    fn set_value(&mut self, k: Self::K, v: Rc<Self::V>);
}


pub trait Trace {
    type U;
    type C: ChoiceBuffer;
    type R;

    fn get_args(&self) -> &Rc<Self::U>;
    fn get_retval(&self) -> &Rc<Self::R>;
    fn get_choices(&self) -> Self::C;
    fn get_score(&self) -> f32;
}


// high-level spec (can't be realized for dynamically-dispatched inference procedures)
pub trait GenerativeFunction {
    type U;
    type C: ChoiceBuffer;
    type T: Trace<U=Self::U>;

    fn simulate(&self, rng: &mut ThreadRng, params: Rc<Self::U>) -> Self::T;
    fn generate(&self, rng: &mut ThreadRng, params: Rc<Self::U>, choices: Self::C) -> Self::T;

    // current assumption: no changes to input arguments
    fn update(&self, trace: Self::T, fwd_choices: Self::C) -> Self::T;
    fn revert(&self, trace: Self::T, bwd_choices: Self::C) -> Self::T;
}