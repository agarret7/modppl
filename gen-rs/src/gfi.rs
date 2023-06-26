use rand::rngs::ThreadRng;
use std::ops::Index;
use std::rc::Rc;
use std::any::Any;


pub type Addr = &'static str;
pub trait ChoiceVal : Any { }

pub trait ChoiceBuffer : Clone + Index<Addr> {
    fn has_value(&self, k: Addr) -> bool;
    fn get_value(&self, k: Addr) -> &Rc<impl ChoiceVal>;
    fn set_value(&mut self, k: Addr, v: &Rc<impl ChoiceVal>);
}

pub trait Trace {
    type X;
    type T;

    fn get_args(&self) -> Rc<Self::X>;
    fn get_retval(&self) -> Rc<Self::T>;
    fn get_choices(&self) -> impl ChoiceBuffer;
    fn get_score(&self) -> f32;
}

pub trait GenerativeFunction {
    type X;
    type T;
    type U: Trace<T=Self::T>;

    fn simulate(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> Self::U;
    fn generate(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> Self::U;

    fn propose(&self, rng: &mut ThreadRng, args: Rc<Self::X>) -> (impl ChoiceBuffer, f32);
    fn assess(&self, rng: &mut ThreadRng, args: Rc<Self::X>, constraints: impl ChoiceBuffer) -> f32;

    // current assumption: no changes to input arguments
    fn update(&self, trace: Rc<Self::U>, constraints: impl ChoiceBuffer) -> (Self::U, impl ChoiceBuffer);
}