/// Representation of the probabilistic execution of a `GenFn`.
#[derive(Clone)]
pub struct Trace<Args,Data,Ret> {
    /// Input arguments to the `GenFn`.
    pub args: Args,

    /// Random variables sampled by the `GenFn`.
    pub data: Data,

    /// The return value of the `GenFn`.
    /// Always `Some(v)` if the `Trace` is returned by a GFI method.
    pub retv: Option<Ret>,

    /// The log joint probability of all the data `log[p(data; args)]`.
    pub logjp: f64
}


impl<Args,Data,Ret> Trace<Args,Data,Ret> {
    /// Create a `Trace` with a `Some(retv)`.
    pub fn new(args: Args, data: Data, retv: Ret, logjp: f64) -> Self {
        Trace { args, data, retv: Some(retv), logjp }
    }

    /// Set `self.retv` to `Some(v)`.
    pub fn set_retv(&mut self, v: Ret) { self.retv = Some(v); }
}


/// Interface for functions that support the standard inference library.
/// 
/// Implementation follows closely to the Generative Function Interface (GFI), as specified in:
/// 
/// > Gen: A General-Purpose Probabilistic Programming System with Programmable Inference.
/// > Cusumano-Towner, M. F.; Saad, F. A.; Lew, A.; and Mansinghka, V. K.
/// > In Proceedings of the 40th ACM SIGPLAN Conference on Programming Language
/// > Design and Implementation (PLDI ‘19).
/// 
/// Any function that implements `GenFn` can use the standard inference library
/// to perform Bayesian inference to generate fair samples from the posterior distribution.
///     
/// `p(trace) ~ p( . | constraints)`
/// 
/// This terminology may be slightly unusual to users from other languages;
/// `data` refers to all random variables, and `constraints` more precisely
/// refers to a subset of the data that we observe. 
pub trait GenFn<Args,Data,Ret> {

    /// Execute the generative function and return a sampled trace.
    fn simulate(&self, args: Args) -> Trace<Args,Data,Ret>;

    /// Execute the generative function consistent with `constraints`.
    /// Return the log probability 
    ///     `log[p(t; args) / q(t; constraints, args)]`
    fn generate(&self, args: Args, constraints: Data) -> (Trace<Args,Data,Ret>, f64);

    /// Update a trace
    fn update(&self,
        trace: Trace<Args,Data,Ret>,
        args: Args,
        diff: GfDiff,
        constraints: Data                    // Data := forward choices
    ) -> (Trace<Args,Data,Ret>, Data, f64);  // Data := backward choices

    /// Call a generative function and return the output.
    fn call(&self, args: Args) -> Ret {
        self.simulate(args).retv.unwrap()
    }

    /// Use a generative function to propose some data.
    fn propose(&self, args: Args) -> (Data, f64) {
        let trace = self.simulate(args);
        (trace.data, trace.logjp)
    }

    /// Assess the conditional probability of some proposed `constraints` under a generative function.
    fn assess(&self, args: Args, constraints: Data) -> f64 {
        let (_, weight) = self.generate(args, constraints);
        weight
    }

}


/// Flag that gives information about the type of incremental difference a generative
/// function can expect to a `Trace`'s arguments during an update.
/// 
/// Can be used to increase efficiency for example in particle filter procedures.
#[derive(Debug,Clone,PartialEq)]
pub enum GfDiff {
    /// No change to input arguments.
    NoChange,

    /// An unknown change to input arguments.
    Unknown,

    /// An incremental change to input arguments.
    /// 
    /// Generally means the `trace` has a vector-valued
    /// `data` field that is being pushed to.
    Extend
}