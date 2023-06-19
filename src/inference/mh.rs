use std::rc::Rc;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;
use crate::{
    types_2d,
    ChoiceBuffer, Trace, GenerativeFunction,
    modeling::{
        dists::{self, Distribution},
    }
};

pub fn metropolis_hastings<X,Y,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    trace: U,
    proposal: impl GenerativeFunction<X=(U,Y)>,
    proposal_args: Rc<Y>,
    observations: impl ChoiceBuffer
) -> (impl Trace<X=X,T=T>, bool) {
    let model_args = trace.get_args();
    // let proposal_args_forward = 
    (trace, false)
}

pub fn mh<X,Y,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    trace: U,
    proposal: impl GenerativeFunction<X=(U,Y)>,
    proposal_args: Rc<Y>,
    observations: impl ChoiceBuffer
) -> (impl Trace<X=X,T=T>, bool) {
    metropolis_hastings(rng, trace, proposal, proposal_args, observations)
}