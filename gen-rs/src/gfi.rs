use rand::rngs::ThreadRng;


#[derive(Clone)]
pub struct Trace<Args,Data,Ret> {
    pub args: Args,
    pub data: Data,
    pub retv: Option<Ret>,
    pub logp: f64
}


impl<Args: 'static,Data: 'static,Ret: 'static> Trace<Args,Data,Ret> {
    pub fn new(args: Args, data: Data, retv: Ret, logp: f64) -> Self {
        Trace { args, data, retv: Some(retv), logp }
    }

    pub fn get_args(&self) -> &Args { &self.args }
    pub fn get_data(&self) -> &Data { &self.data }
    pub fn get_data_mut(&mut self) -> &mut Data { &mut self.data }
    pub fn get_retv(&self) -> Option<&Ret> { self.retv.as_ref() }
    pub fn set_retv(&mut self, v: Ret) { self.retv = Some(v); }
    pub fn logpdf(&self) -> f64 { self.logp }
}

pub trait GenFn<Args,Data,Ret> {
    fn rng(&self) -> ThreadRng;
    fn simulate(&mut self, args: Args) -> Trace<Args,Data,Ret>;
    fn generate(&mut self, args: Args, constraints: Data) -> (Trace<Args,Data,Ret>, f64);

    fn update(&mut self,
        trace: &mut Trace<Args,Data,Ret>,
        args: Args,
        diff: GfDiff,
        constraints: Data  // forward choices
    ) -> (Data, f64);      // backward choices


    fn call(&mut self, args: Args) -> Ret {
        self.simulate(args).retv.unwrap()
    }

    fn propose(&mut self, args: Args) -> (Data, f64) {
        let trace = self.simulate(args);
        (trace.data, trace.logp)
    }

    fn assess(&mut self, args: Args, constraints: Data) -> f64 {
        self.generate(args, constraints).1
    }
}


// TODO: extend the semantics to support variable-length input and per-argument diffs.
#[derive(Debug,Clone)]
pub enum GfDiff {
    NoChange,
    Unknown,
    Extend
}