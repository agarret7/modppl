pub fn metropolis_adjusted_langevin_ascent<X: Copy,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: U,
    addr: Addr,
    tau: f64
) -> (U, bool) {
    let bwd_choices = trace.get_choices();
    let old_score = trace.get_score();

    let trace = Rc::new(trace);
    let std = (2 * tau).sqrt();
    let log_sqrt_2_pi = 0.5*(2*PI).ln();
    let log_density = -std.ln() - log_sqrt_2_pi;

    let proposal_mean = trace.get_choices()[addr];

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

pub fn mala<X: Copy,T,U: Trace<X=X,T=T>>(
    rng: &mut ThreadRng,
    model: &impl GenerativeFunction<X=X,T=T,U=U>,
    trace: U,
    addr: Addr,
    tau: f64
) -> (U, bool) {
    metropolis_adjusted_langevin_ascent(rng, model, trace, addr, tau)
}