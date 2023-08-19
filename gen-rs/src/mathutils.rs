/// For an input vector of `[x1, ..., xn]`, return `log(exp(x1) + ... + exp(xn))`.
pub fn logsumexp(xs: &Vec<f64>) -> f64 {
    let max = xs.iter().cloned().fold(-1./0. /* -inf */, f64::max);
    if max == f64::NEG_INFINITY {
        f64::NEG_INFINITY
    } else {
        let mut sum_exp = 0.;
        for x in xs {
            sum_exp += (x - max).exp();
        }
        max + sum_exp.ln()
    }
}
