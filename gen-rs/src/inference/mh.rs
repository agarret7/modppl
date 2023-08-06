use std::rc::{Rc,Weak};
use approx;
use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;
use crate::{Trace,GenFn,GfDiff::NoChange};


pub fn metropolis_hastings<Args: Clone + 'static,Data: Clone + 'static,Ret: 'static,ProposalArgs: Clone>(
    model: &mut impl GenFn<Args,Data,Ret>,
    trace: Trace<Args,Data,Ret>,
    proposal: &mut impl GenFn<(Weak<Trace<Args,Data,Ret>>,ProposalArgs),Data,()>,
    proposal_args: ProposalArgs
) -> (Trace<Args,Data,Ret>, bool) {
    let old_logp = trace.logpdf();
    let bwd_choices = trace.data.clone();

    let trace = Rc::new(trace);
    let proposal_args_forward = (Rc::downgrade(&trace), proposal_args.clone());
    let (fwd_choices, fwd_weight) = proposal.propose(proposal_args_forward);
    let mut trace = Rc::into_inner(trace).unwrap();

    let args = trace.args.clone();
    let (discard, weight) = model.update(&mut trace, args.clone(), NoChange, fwd_choices);

    let trace = Rc::new(trace);
    let proposal_args_backward = (Rc::downgrade(&trace), proposal_args);
    let bwd_weight = proposal.assess(proposal_args_backward, discard);
    let mut trace = Rc::into_inner(trace).unwrap();

    dbg!(weight);
    // dbg!(old_score);
    dbg!(fwd_weight);
    dbg!(bwd_weight);
    // dbg!(new_score);

    let alpha = weight - fwd_weight + bwd_weight;
    if model.rng().sample(Uniform::new(0_f64, 1_f64)).ln() < alpha {
        (trace, true)
    } else {
        model.update(&mut trace, args, NoChange, bwd_choices);
        let revert_logp: f64;
        if trace.logpdf() == f64::NEG_INFINITY {
            revert_logp = old_logp;
        } else {
            revert_logp = trace.logpdf();
        }
        trace.logp = revert_logp;
        approx::assert_abs_diff_eq!(trace.logp, old_logp, epsilon = 1e-8);
        (trace, false)
    }
}

// pub fn mh<X: Copy,Y: Copy,T,U: Trace<X=X,T=T>>(
//     rng: &mut ThreadRng,
//     model: &impl GenerativeFunction<X=X,T=T,U=U>,
//     trace: U,
//     proposal: &impl GenerativeFunction<X=(Weak<U>,Y),U=U>,
//     proposal_args: Y
// ) -> (U, bool) {
//     metropolis_hastings(rng, model, trace, proposal, proposal_args)
// }