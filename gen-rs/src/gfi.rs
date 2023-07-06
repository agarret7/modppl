use rand::rngs::ThreadRng;
use std::rc::Rc;
use std::any::Any;


pub type Addr = &'static str;


pub trait ChoiceBuffer : Clone {
    type V: Any;

    fn has_value(&self, k: Addr) -> bool;
    fn get_value(&self, k: Addr) -> &Rc<Self::V>;
    fn set_value(&mut self, k: Addr, v: &Rc<Self::V>);
}


pub trait Trace {
    type X;
    type T;

    fn get_args(&self) -> &Self::X;
    fn get_retval(&self) -> &Self::T;
    fn get_choices(&self) -> impl ChoiceBuffer;
    fn get_score(&self) -> f64;
    fn set_score(&mut self, new_score: f64);
}


pub trait GenerativeFunction {
    type X;
    type T;
    type U: Trace<T=Self::T>;

    fn simulate(&self, rng: &mut ThreadRng, args: Self::X) -> Self::U;
    fn generate(&self, rng: &mut ThreadRng, args: Self::X, constraints: impl ChoiceBuffer) -> Self::U;

    fn propose(&self, rng: &mut ThreadRng, args: Self::X) -> (impl ChoiceBuffer, f64);
    fn assess(&self, rng: &mut ThreadRng, args: Self::X, constraints: impl ChoiceBuffer) -> f64;

    // current assumption: no changes to input arguments
    fn update(&self, rng: &mut ThreadRng, trace: &mut Self::U, args: Self::X, diff: GfDiff, constraints: impl ChoiceBuffer) -> impl ChoiceBuffer;
}


// TODO: extend the semantics to support per-argument diffs. This is challenging. See:
// - https://soasis.org/posts/a-mirror-for-rust-a-plan-for-generic-compile-time-introspection-in-rust/#variadics-do-not-exist-in-rust 
// - https://internals.rust-lang.org/t/analysis-pre-rfc-variadic-generics-in-rust/13879
#[derive(Debug,Clone)]
pub enum GfDiff {
    NoChange,
    Unknown,
    Extend
}