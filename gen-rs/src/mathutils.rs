pub fn logsumexp(xs: &Vec<f32>) -> f32 {
    let max = xs.iter().cloned().fold(-1./0. /* -inf */, f32::max);
    const NEGATIVE_INFINITY: f32 = -f32::INFINITY;
    if max == NEGATIVE_INFINITY {
        return NEGATIVE_INFINITY
    } else {
        let mut sum_exp = 0.;
        for x in xs {
            sum_exp += (x - max).exp();
        }
        return max + sum_exp.ln();
    }
}
