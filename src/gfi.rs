use rand::rngs::ThreadRng;
use std::rc::Rc;
use std::any::Any;

// use crate::ChoiceHashMap;
use crate::mathutils::logsumexp;


pub type Addr = &'static str;
pub trait ChoiceVal : Any { }

pub trait ChoiceBuffer : Clone {
    fn has_value(&self, k: Addr) -> bool;
    fn get_value(&self, k: Addr) -> &Rc<impl ChoiceVal>;
    fn set_value(&mut self, k: Addr, v: &Rc<impl ChoiceVal>);
}


pub trait Trace {
    type X;
    type T;

    fn get_args(&self) -> &Rc<Self::X>;
    fn get_retval(&self) -> &Rc<Self::T>;
    fn get_choices(&self) -> impl ChoiceBuffer;
    fn get_score(&self) -> f32;
}

// high-level spec (can't be realized for dynamically-dispatched inference procedures)
pub trait GenerativeFunction {
    type X;
    type T;
    type U: Trace<X=Self::X,T=Self::T>;

    fn simulate(&self, rng: &mut ThreadRng, params: Rc<Self::X>) -> Self::U;
    fn generate(&self, rng: &mut ThreadRng, params: Rc<Self::X>, choices: impl ChoiceBuffer) -> Self::U;

    // current assumption: no changes to input arguments
    fn update(&self, trace: Self::U, fwd_choices: impl ChoiceBuffer) -> Self::U;
    fn revert(&self, trace: Self::U, bwd_choices: impl ChoiceBuffer) -> Self::U;
}