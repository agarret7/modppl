// equivalent implementation of the example model on Ferric's README:
// https://github.com/ferric-ai/ferric

use modppl::prelude::*;

dyngen!(
pub fn grass() -> (bool,bool) {
    let rain = bernoulli(0.2) %= "rain";

    let sprinkler = if rain {
        bernoulli(0.01) %= "sprinkler"
    } else {
        bernoulli(0.4) %= "sprinkler"
    };

    let grass_wet = bernoulli(
        if sprinkler && rain { 0.99 }
        else if sprinkler && !rain { 0.9 }
        else if !sprinkler && rain { 0.8 }
        else { 0.0 }
    ) %= "grass_wet";

    (rain, sprinkler)
});