use std::rc::Rc;
use ndarray::{Array1,Array2};

pub struct ParticleFilterState {
    transition: Array2<f32>,
    obs_model: Array2<f32>,

    process_cov: Array2<f32>,
    obs_cov: Array2<f32>,

    latents: Array1<Rc<Array1<f32>>>,
    observations: Array1<Rc<Array1<f32>>>
}

// impl Trace for ParticleFilterState {
//     type X;
//     type T;

//     fn get_args(&self) -> Rc<Self::X>;
//     fn get_retval(&self) -> Rc<Self::T>;
//     fn get_choices(&self) -> impl ChoiceBuffer;
//     fn get_score(&self) -> f32;
// }