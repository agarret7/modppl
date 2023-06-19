use rand::{Rng, rngs::ThreadRng};
use rand::distributions::Uniform;
use crate::{
    types_2d,
    Trace,
    modeling::{
        dists::{self, Distribution},
    }
};


const X_STD_MH: f32 = 0.02;
const Y_STD_MH: f32 = 0.02;


pub fn metropolis_hastings<U,C,R>(rng: &mut ThreadRng, trace: dyn Trace<U,C,R=R>) -> dyn Trace<U,C,R=R> {
    // propose the new point from a local gaussian drift
    let last_point = trace.get_choices().0.last().unwrap();
    let proposed_x = dists::normal.random(rng, &(last_point.x, X_STD_MH));
    let proposed_y = dists::normal.random(rng, &(last_point.y, Y_STD_MH));
    let proposed_point = types_2d::Point { x: proposed_x, y: proposed_y };

    // dbg!(last_point);
    // dbg!(&proposed_point);

    // update the trace with the proposed latent
    let new_trace = trace.update(proposed_point);

    // normally we'd have to do this for the MH acceptance probability:

    // calculate the forward weight
    // let fwd_weight = dists::normal.logpdf(&proposed_x, &(last_point.x, X_STD_MH))
    //     + dists::normal.logpdf(&proposed_y, &(last_point.y, Y_STD_MH));

    // calculate the backward weight
    // let bwd_weight = dists::normal.logpdf(&last_point.x, &(proposed_x, X_STD_MH))
    //     + dists::normal.logpdf(&last_point.y, &(proposed_y, Y_STD_MH));

    // let alpha = (new_trace.get_score() - trace.get_score() +
    //     bwd_weight - fwd_weight).exp();

    // but we can optimize our calculation like this
    // because of symmetry in the Gaussian proposal:
    let alpha = (new_trace.get_score() - trace.get_score()).exp();

    let obs = trace.get_retval().last().unwrap();
    // dbg!(obs);
    let prev_dist = ((obs.x - last_point.x).powf(2.) + (obs.y - last_point.y).powf(2.)).sqrt();
    dbg!(prev_dist);
    dbg!(trace.get_score());

    let proposed_dist = ((obs.x - proposed_x).powf(2.) + (obs.y - proposed_y).powf(2.)).sqrt();
    dbg!(proposed_dist);
    dbg!(new_trace.get_score());

    // accept or reject (always accepts if alpha > 1)
    if alpha > rng.sample(Uniform::new(0.,1.)) {
        return Some(new_trace)
    } else {
        return None
    }
}