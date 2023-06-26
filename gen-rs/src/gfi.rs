use rand::rngs::ThreadRng;
use std::ops::Index;
use std::rc::Rc;
use std::any::Any;


pub type Addr = &'static str;

pub trait ChoiceBuffer : Clone + Index<Addr> {
    type V: Any;

    fn has_value(&self, k: Addr) -> bool;
    fn get_value(&self, k: Addr) -> &Rc<Self::V>;
    fn set_value(&mut self, k: Addr, v: &Rc<Self::V>);
}

pub trait Trace {
    type X;
    type T;

    fn get_args(&self) -> Rc<Self::X>;
    fn get_retval(&self) -> Rc<Self::T>;
    fn get_choices(&self) -> impl ChoiceBuffer;
    fn get_score(&self) -> f64;
}

pub trait GenerativeFunction {
    type X;
    type T;
    type U: Trace<T=Self::T>;

    fn simulate(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> Self::U;
    fn generate(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> Self::U;

    fn propose(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> (impl ChoiceBuffer, f64);
    fn assess(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> f64;

    // current assumption: no changes to input arguments
    fn update(&self, trace: Rc<Self::U>, constraints: impl ChoiceBuffer) -> (Self::U, impl ChoiceBuffer);
}