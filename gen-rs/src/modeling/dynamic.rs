// type TrieTrace<A,T> = StackTrace<A,Addr<,T>;
// type VecTrace<A,D: Addr<str>,T> = StackTrace<A,Vec<D>,Vec<T>>;

// struct TrieFn<A,T>();

// struct TrieBuilder {
// }

use rand::rngs::ThreadRng;

use crate::modeling::dists::u01;
use crate::{GenFn,GfDiff,Addr, gfi_new::{Trace, TraceBox}, StrRec, Sample};

pub enum DynGenFnState<A,S: Addr<V=TraceBox>,T> {
    Simulate { trace: Trace<A,S,T> }
}

use std::time::Instant;

impl<A: 'static,S: Addr<V=TraceBox> + 'static,T: 'static> DynGenFnState<A,S,T> {
    fn traceat<
        X: 'static,
        Y: Addr + 'static,
        Z: 'static + Clone
    >(&mut self, mut gen_fn: impl GenFn<X,Y,Z>, args: X, key: StrRec) -> Z {
        match self {
            DynGenFnState::Simulate { ref mut trace } => {
                let now = Instant::now();
                let subtrace = gen_fn.simulate(args);
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);

                trace.logp += subtrace.logp;
                let retv = subtrace.get_retv().unwrap().clone();

                let data = trace.get_data_mut();

                let b_val = TraceBox::from_trace(subtrace);

                // this call is very inefficient due to regex parsing
                let now = Instant::now();
                data.insert_value(key, b_val);
                let elapsed = now.elapsed();
                println!("Elapsed: {:.2?}", elapsed);

                retv
            }
        }
    }
}

use crate::{Trie,modeling::dists::normal};

pub struct DynGenFn<A,S: Addr<V=TraceBox>,T> {
    rng: ThreadRng,
    func: fn(&mut DynGenFnState<A,S,T>, A) -> T,
}

impl<A,S: Addr<V=TraceBox>,T> DynGenFn<A,S,T>{
    pub fn new(func: fn(&mut DynGenFnState<A,S,T>, A) -> T) -> Self {
        DynGenFn {
            rng: ThreadRng::default(),
            func
        }
    }
}


pub struct NormalGF { }
pub const normal_gf: NormalGF = NormalGF { };
use std::f64::consts::PI;

impl GenFn<(f64,f64),Sample<f64>,f64> for NormalGF {
    fn rng(&self) -> ThreadRng { ThreadRng::default() }

    fn simulate(&mut self, args: (f64,f64)) -> Trace<(f64,f64),Sample<f64>,f64> {
        // value
        let (mu, std) = args;
        let mut u: f64 = 0.;
        let mut r: f64 = 0.;
        while r == 0. || r > 1. {
            u = u01(&mut self.rng()) * 2. - 1.;
            let v: f64 = u01(&mut self.rng()) * 2. - 1.;
            r = u * u + v * v;
        }
        let c = (-2. * r.ln() / r).sqrt();
        let x = u * c * std + mu;

        // logpdf
        let z = (x - mu) / std;
        let logp = -(z.abs().powf(2.) + (2.*PI).ln())/2. - std.ln();

        Trace::new(args, Sample(x), x, logp)
    }
    fn generate(&mut self, args: (f64,f64), constraints: impl Addr) -> (Trace<(f64,f64),Sample<f64>,f64>, f64) {
        panic!("not implemented!");
    }
    fn update(&mut self,
            trace: &mut Trace<(f64,f64),Sample<f64>,f64>,
            args: (f64,f64),
            diff: crate::GfDiff,
            constraints: impl Addr  // forward choices
        ) -> (Sample<f64>, f64) {
        panic!("not implemented!");
    }
}

// DynGenFnState<f64,(f64,f64),f64>

pub fn test_model(state: &mut DynGenFnState<f64,Trie<TraceBox>,f64>, noise: f64) -> f64 {
    let mut sum = 0.;
    for i in (1..100).into_iter() {
        let x = state.traceat(normal_gf, (1., noise), Box::leak(format!("{}", i).into_boxed_str()));
        sum += x;
    }
    sum
}

#[test]
pub fn test_dynamic_model_prototype() {
    let mut dynamic_model_prototype = DynGenFn::new(test_model);

    for i in (0..100).into_iter() {
        let trace = dynamic_model_prototype.simulate(1.);
        // let data = trace.get_data();
        // dbg!(data.get_value("x").unwrap().into_inner::<(f64,f64),Sample<f64>,f64>().get_retv());
        dbg!(trace.get_retv().unwrap());
        dbg!(trace.logpdf());
    }
}

// A DynGenFn is a generative function that constructs its stack-trace during
// runtime (allocating all internal memory dynamically on the heap).
impl<A: Clone + 'static,S: Addr<V=TraceBox> + 'static,T: 'static> GenFn<A,S,T> for DynGenFn<A,S,T> {

    fn rng(&self) -> ThreadRng { self.rng.clone() }
    fn simulate(&mut self, args: A) -> Trace<A,S,T> {
        let mut state = DynGenFnState::Simulate { trace: Trace::<A,S,T>::empty(args.clone()) };
        let retv = (self.func)(&mut state, args);
        let DynGenFnState::Simulate {mut trace} = state;
        trace.set_retv(retv);
        trace
    }
    fn generate(&mut self, args: A, constraints: impl Addr) -> (Trace<A,S,T>, f64) {
        panic!("not implemented")
    }

    fn update(&mut self,
        trace: &mut Trace<A,S,T>,
        args: A,
        diff: GfDiff,
        constraints: impl Addr
    ) -> (S, f64) {
        panic!("not implemented")
    }      // backward choices

    // fn call(&mut self, args: Self::A) -> Self::T;
    // fn propose(&mut self, args: Self::A) -> (impl Addr<str>, f64);
    // fn assess(&mut self, args: Self::A, constraints: impl Addr<str>) -> f64;
}