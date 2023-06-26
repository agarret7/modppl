use std::rc::Rc;
use ndarray::{Array1,Array2};

pub struct ParticleFilterState {
    transition: Array2<f64>,
    obs_model: Array2<f64>,

    process_cov: Array2<f64>,
    obs_cov: Array2<f64>,

    latents: Array1<Rc<Array1<f64>>>,
    observations: Array1<Rc<Array1<f64>>>
}

// impl Trace for ParticleFilterState {
//     type X;
//     type T;

//     fn get_args(&self) -> Rc<Self::X>;
//     fn get_retval(&self) -> Rc<Self::T>;
//     fn get_choices(&self) -> impl ChoiceBuffer;
//     fn get_score(&self) -> f64;
// }