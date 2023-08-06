// The core insight is that optimizing a probabilistic program is really about optimizing
// searchable _memory structure_ given deterministic constraints on particular (known) data queries.

// Thus, constraints on generative function DSLs express constraints on the structure type
// used to represent call and choice values on subtraces.

// In Gen.jl:
// - Args StaticDSL program infers optimized codegen for structs and constructors, given flat randomness.
// - Args DynamicDSL program infers optimized trie constructors, given recursive hierarchical randomness.
// - The Unfold combinator infers optimized vector constructors, given sequential randomness.

// Reflecting this pattern, gen-rs forgoes indirection via Choice and Trace interfaces,
// directly exposing the data type as parameterizing the domain specific language.

// This expands the {Static | Dynamic} DSLs into the {struct | HashMap, Trie*, MerkleTree & Vec**} DSLs.
// *Trie is the default when annotations are omitted.
// **Vec replaces the Unfold combinator from Gen.jl.

// GenMerkle demos
// Verify two large traces contain the same data

// Since each of these structures can implement the Index trait,
// choices are represented as implementations of the Index trait (`impl Index`).

// Args user can compose languages to express an eg. "array-of-tries" pattern or
// "trie-of-structs"

// gen!(Trie fn trie_model(x: f64, y: f64) -> bool {
//     z ~ bernoulli(-(y - x).abs().exp());
// });

// gen!(Vec fn model(Self::Ret: int) -> Vec<bool> {
//     let a ~ beta(2., 5.);
//     let bs = vec![]
//     {i} = (1..Self::Ret).into_iter() {
//         let t = i;
//         dbg!(t);
//         let x = {x} ~ normal(0, a);
//         let y = {y} ~ normal(a, a^2);
//         let b = {b} ~ trie_model(x, y);
//         bs.push(b);
//     }
//     bs
// });


use rand::rngs::ThreadRng;


// pub trait Argsddr: Index<StrRec> + Sized {
//     type V;
//     fn empty() -> Self;
//     fn get_submap(&self, addr: StrRec) -> Option<&Self>;
//     fn insert_submap(&mut self, addr: StrRec, submap: Self) -> Option<Self>;
//     fn remove_submap(&mut self, addr: StrRec) -> Option<Self>;
//     fn get_value(&self, addr: StrRec) -> Option<&Self::V>;
//     fn insert_value(&mut self, addr: StrRec, value: Self::V) -> Option<Self::V>;
//     fn remove_value(&mut self, addr: StrRec) -> Option<Self::V>;
// }


// impl<Self::Ret> Index<StrRec> for Sample<Self::Ret> {
//     type Ret = Self::Ret;

//     fn index(&self, _: StrRec) -> &Self::Ret {
//         &self.0
//     }
// }

// impl<Self::Ret> Argsddr for Sample<Self::Ret> {
//     type V = Self::Ret;
//     fn empty() -> Self { panic!("samples can't be empty") }
//     fn get_submap(&self, _: StrRec) -> Option<&Self> { panic!("samples don't have submaps") }
//     fn insert_submap(&mut self, _: StrRec, _: Self) -> Option<Self> { panic!("samples don't have submaps") }
//     fn remove_submap(&mut self, addr: StrRec) -> Option<Self> { panic!("samples don't have submaps") }
//     fn get_value(&self, _: StrRec) -> Option<&Self::V> { Some(&self.0) }
//     fn insert_value(&mut self, _: StrRec, value: Self::V) -> Option<Self::V> {
//         Some(std::mem::replace(&mut self.0, value))
//     }
//     fn remove_value(&mut self, addr: StrRec) -> Option<Self::V> { Some(self.0) }
// }

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