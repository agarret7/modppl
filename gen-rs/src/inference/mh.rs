use std::rc::{Rc,Weak};
use approx;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;
use crate::{Trace,GenerativeFunction,GfDiff::NoChange};


pub fn metropolis_hastings<X: Copy,Y: Copy,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: U,
    proposal: &impl GenerativeFunction<X=(Weak<U>,Y),U=U>,
    proposal_args: Y
) -> (U, bool) {
    let bwd_choices = trace.get_choices();
    let old_score = trace.get_score();

    let trace = Rc::new(trace);
    let proposal_args_forward = (Rc::downgrade(&trace), proposal_args);
    let (fwd_choices, fwd_weight) = proposal.propose(rng, proposal_args_forward);
    let mut trace = Rc::into_inner(trace).unwrap();

    let args = *trace.get_args();
    let discard = model.update(rng, &mut trace, args, NoChange, fwd_choices);
    let new_score = trace.get_score();

    let trace = Rc::new(trace);
    let proposal_args_backward = (Rc::downgrade(&trace), proposal_args);
    let bwd_weight = proposal.assess(rng, proposal_args_backward, discard);
    let mut trace = Rc::into_inner(trace).unwrap();

    // dbg!(old_score);
    // dbg!(fwd_weight);
    // dbg!(bwd_weight);
    // dbg!(new_score);

    let alpha = new_score - fwd_weight + bwd_weight - old_score;
    if rng.sample(Uniform::new(0_f64, 1_f64)).ln() < alpha {
        (trace, true)
    } else {
        model.update(rng, &mut trace, args, NoChange, bwd_choices);
        let revert_score: f64;
        if new_score == f64::NEG_INFINITY {
            revert_score = old_score;
        } else {
            revert_score = trace.get_score();
        }
        trace.set_score(revert_score);
        approx::assert_abs_diff_eq!(trace.get_score(), old_score, epsilon = 1e-8);
        (trace, false)
    }
}

pub fn mh<X: Copy,Y: Copy,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: U,
    proposal: &impl GenerativeFunction<X=(Weak<U>,Y),U=U>,
    proposal_args: Y
) -> (U, bool) {
    metropolis_hastings(rng, model, trace, proposal, proposal_args)
}