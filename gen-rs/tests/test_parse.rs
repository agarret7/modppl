use gen_rs::prelude::*;


dyngen!(
fn hyperprior(a: f64, b: f64) -> bool {
    let p = beta(a,b) %= "prob_is_small";
    bernoulli(p) %= "is_small"
});

dyngen!(
fn model() -> f64 {
    if hyperprior(2.,2.) /= "var" {
        normal(0.,0.05) %= "y"
    } else {
        normal(0.,1.0) %= "y"
    }
});

dyngen!(
fn proposal(tr: Weak<DynTrace<(),f64>>, drift: f64, addr: String) {
    let tr = tr.upgrade().unwrap();
    normal(tr.data.read::<f64>(&addr), drift) %= &addr;
});

#[test]
pub fn test_parse() {
    let mut constraints = DynTrie::new();
    constraints.observe("y", Arc::new(0.3));
    let mut tr = model.simulate(());
    for _ in 0..1000 {
        let (new_tr, accepted) = mh(&model, tr, &proposal, (0.5,String::from("var/prob_is_small")));
        dbg!(accepted);
        tr = new_tr;
    }
}
