use std::rc::{Rc,Weak};
use approx;
use rand::{Rng, distributions::Uniform};
use crate::{GLOBAL_RNG,Trace,GenFn,GfDiff::NoChange};


/// Perform a Metropolis-Hastings update that proposes new values for some subset of random choices in the given `trace` under the `model` using the given `proposal` generative function.
/// 
/// The `proposal` shares the same trace data structure as the `model`, but must accept a `Weak` reference to the `trace` as its first argument and return an empty tuple `()`.
pub fn metropolis_hastings<Args: Clone + 'static,Data: Clone + 'static,Ret: 'static,ProposalArgs: Clone>(
    model: &impl GenFn<Args,Data,Ret>,
    trace: Trace<Args,Data,Ret>,
    proposal: &impl GenFn<(Weak<Trace<Args,Data,Ret>>,ProposalArgs),Data,()>,
    proposal_args: ProposalArgs
) -> (Trace<Args,Data,Ret>, bool) {
    let old_logp = trace.logp;
    let bwd_choices = trace.data.clone();

    let trace = Rc::new(trace);
    let proposal_args_forward = (Rc::downgrade(&trace), proposal_args.clone());
    let (fwd_choices, fwd_weight) = proposal.propose(proposal_args_forward);
    let trace = Rc::into_inner(trace).unwrap();

    let args = trace.args.clone();
    let (trace, discard, weight) = model.update(trace, args.clone(), NoChange, fwd_choices);

    let trace = Rc::new(trace);
    let proposal_args_backward = (Rc::downgrade(&trace), proposal_args);
    let bwd_weight = proposal.assess(proposal_args_backward, discard);
    let mut trace = Rc::into_inner(trace).unwrap();

    dbg!(weight);
    dbg!(fwd_weight);
    dbg!(bwd_weight);

    let alpha = weight - fwd_weight + bwd_weight;
    if GLOBAL_RNG.with_borrow_mut(|rng| rng.sample(Uniform::new(0_f64, 1_f64)).ln()) < alpha {
        (trace, true)
    } else {
        (trace, _, _) = model.update(trace, args, NoChange, bwd_choices);
        if !trace.logp.is_finite() {
            trace.logp = old_logp;
        }
        approx::assert_abs_diff_eq!(trace.logp, old_logp, epsilon = 1e-8);
        (trace, false)
    }
}

/// Alias for `metropolis_hastings`.
pub fn mh<Args: Clone + 'static,Data: Clone + 'static,Ret: 'static,ProposalArgs: Clone>(
    model: &impl GenFn<Args,Data,Ret>,
    trace: Trace<Args,Data,Ret>,
    proposal: &impl GenFn<(Weak<Trace<Args,Data,Ret>>,ProposalArgs),Data,()>,
    proposal_args: ProposalArgs
) -> (Trace<Args,Data,Ret>, bool) {
    metropolis_hastings(model, trace, proposal, proposal_args)
}