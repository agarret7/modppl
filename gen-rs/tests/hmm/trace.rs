use std::rc::Rc;
use gen_rs::{ChoiceBuffer,ChoiceHashMap,Trace};


#[derive(Clone,Copy)]
pub struct ParamStore { }

#[derive(Clone)]
pub struct HMMTrace {
    args: (i64, ParamStore),
    states: Vec<Rc<usize>>,
    observations: Vec<Rc<usize>>,
    score: f64
}

impl HMMTrace {
    fn validate(&self) -> () {
        assert_eq!(self.states.len() as i64, self.args.0);
        assert_eq!(self.observations.len() as i64, self.args.0);
    }

    pub fn new() -> Self {
        HMMTrace {
            args: (0, ParamStore { }),
            states: vec![].into(),
            observations: vec![].into(),
            score: 0.0,
        }
    }

    pub fn extend(&mut self, new_state: usize, new_observation: usize) {
        // warning: this may invalidate the score
        // you probably want to call self.set_score after this
        self.states.push(Rc::new(new_state));
        self.observations.push(Rc::new(new_observation));
        self.args.0 += 1;
    }

    pub fn get_t(&self) -> i64 {
        self.validate();
        self.args.0
    }
}

impl Trace for HMMTrace {
    type X = (i64, ParamStore);
    type T = Vec<Rc<usize>>;

    fn get_args(&self) -> &Self::X {
        &self.args
    }

    fn get_retval(&self) -> &Self::T {
        self.validate();
        &self.observations
    }

    fn get_choices(&self) -> ChoiceHashMap<usize> {
        self.validate();
        let mut cmap = ChoiceHashMap::new();
        for t in 1..self.get_t()+1 {
            let idx = (t-1) as usize;
            let state = &self.states[idx];
            let observation = &self.observations[idx];
            cmap.set_value(Box::leak(format!("{} => state", t).into_boxed_str()), state);
            cmap.set_value(Box::leak(format!("{} => observation", t).into_boxed_str()), observation);
        }
        cmap
    }

    fn get_score(&self) -> f64 {
        self.score
    }

    fn set_score(&mut self, new_score: f64) {
        self.score = new_score;
    }
}