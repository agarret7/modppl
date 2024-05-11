use std::sync::{Arc,Weak};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use crate::{Trace,GenFn,GfDiff::NoChange};


/// Perform a Metropolis-Hastings update that proposes new values for some subset of random choices in the given `trace` under the `model` using the given `proposal` generative function.
/// 
/// The `proposal` shares the same trace data structure as the `model`, but must accept a `Weak` reference to the `trace` as its first argument and return an empty tuple `()`.
pub fn metropolis_hastings<Args: Clone + 'static,Data: Clone + 'static,Ret: Clone + 'static,ProposalArgs: Clone>(
    model: &impl GenFn<Args,Data,Ret>,
    trace: Trace<Args,Data,Ret>,
    proposal: &impl GenFn<(Weak<Trace<Args,Data,Ret>>,ProposalArgs),Data,()>,
    proposal_args: ProposalArgs
) -> (Trace<Args,Data,Ret>, bool) {
    let mut prng = ThreadRng::default();
    let prev_trace = trace.clone();

    let trace = Arc::new(trace);
    let proposal_args_forward = (Arc::downgrade(&trace), proposal_args.clone());
    let (fwd_choices, fwd_weight) = proposal.propose(proposal_args_forward);
    let trace = Arc::into_inner(trace).unwrap();

    let args = trace.args.clone();
    let (trace, discard, weight) = model.update(trace, args.clone(), NoChange, fwd_choices);

    let trace = Arc::new(trace);
    let proposal_args_backward = (Arc::downgrade(&trace), proposal_args);
    let bwd_weight = proposal.assess(proposal_args_backward, discard);
    let trace = Arc::into_inner(trace).unwrap();

    // dbg!(weight);
    // dbg!(fwd_weight);
    // dbg!(bwd_weight);

    let alpha = weight - fwd_weight + bwd_weight;
    if prng.sample(Uniform::new(0_f64, 1_f64)).ln() < alpha {
        (trace, true)
    } else {
        (prev_trace, false)
    }
}

/// Alias for `metropolis_hastings`.
pub fn mh<Args: Clone + 'static,Data: Clone + 'static,Ret: Clone + 'static,ProposalArgs: Clone>(
    model: &impl GenFn<Args,Data,Ret>,
    trace: Trace<Args,Data,Ret>,
    proposal: &impl GenFn<(Weak<Trace<Args,Data,Ret>>,ProposalArgs),Data,()>,
    proposal_args: ProposalArgs
) -> (Trace<Args,Data,Ret>, bool) {
    metropolis_hastings(model, trace, proposal, proposal_args)
}