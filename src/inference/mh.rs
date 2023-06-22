use std::rc::Rc;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;
use crate::{Trace,GenerativeFunction};


pub fn metropolis_hastings<X,Y,T,U: Trace<T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: Rc<U>,
    proposal: &impl GenerativeFunction<X=(Rc<U>,Rc<Y>),U=U>,
    proposal_args: Rc<Y>,
) -> (Rc<U>, bool) {
    let proposal_args_forward = (trace.clone(), proposal_args.clone());
    let (fwd_choices, fwd_weight) = proposal.propose(rng, Rc::new(proposal_args_forward));

    let (new_trace, discard) = model.update(trace.clone(), fwd_choices);
    let new_trace = Rc::new(new_trace);

    let proposal_args_backward = (new_trace.clone(), proposal_args.clone());
    let bwd_weight = proposal.assess(rng, Rc::new(proposal_args_backward), discard);

    // dbg!(trace.get_score());
    // dbg!(fwd_weight);
    // dbg!(bwd_weight);
    // dbg!(new_trace.get_score());

    let alpha = new_trace.get_score() - fwd_weight + bwd_weight - trace.get_score();

    if rng.sample(Uniform::new(0_f32, 1_f32)).ln() < alpha {
        (new_trace, true)
    } else {
        (trace, false)
    }
}

pub fn mh<X,Y,T,U: Trace<T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: Rc<U>,
    proposal: &impl GenerativeFunction<X=(Rc<U>,Rc<Y>),U=U>,
    proposal_args: Rc<Y>,
) -> (Rc<U>, bool) {
    metropolis_hastings(rng, model, trace, proposal, proposal_args)
}