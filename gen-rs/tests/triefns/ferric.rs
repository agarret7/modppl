// equivalent implementation of the example model on Ferric's README:
// https://github.com/ferric-ai/ferric

use gen_rs::{TrieFn,TrieFnState, bernoulli};

fn _grass(state: &mut TrieFnState<(),(bool,bool)>, args: ()) -> (bool,bool) {
    let rain = state.sample_at(&bernoulli, 0.2, "rain");

    let sprinkler = if rain {
        state.sample_at(&bernoulli, 0.01, "sprinkler")
    } else {
        state.sample_at(&bernoulli, 0.4, "sprinkler")
    };

    let grass_wet = state.sample_at(&bernoulli,
        if sprinkler && rain { 0.99 }
        else if sprinkler && !rain { 0.9 }
        else if !sprinkler && rain { 0.8 }
        else { 0.0 },
        "grass_wet"
    );

    (rain, sprinkler)
}
const grass: TrieFn<(), (bool, bool)> = TrieFn { func: _grass };