pub mod dists;
pub mod triefn;


// use std::{any::Any, rc::Rc};
// use crate::{Sample,gfi_new::{TraceNew as Trace}};


// pub enum CallSite {
//     TraceAt { args: Rc<dyn Any>, data: Rc<dyn Any>, retv: Option<Rc<dyn Any>>, logp: f64 },
//     SampleAt { x: Rc<dyn Any>, logp: f64 }
// }

// pub struct CallSite {
//     x: Rc<dyn Any>,
//     logp: f64
// }

// impl CallSite {
//     pub fn new<T>(x: T) -> Self {
//         CallSite { Rc::new(x), logp: 0. }
//     }
// }

// impl CallSite {
//     pub fn into_inner<T>(self) -> Rc<T> {
//         match self {
//             Self::TraceAt { args, data, retv, logp } => {
//                 *retv.unwrap().downcast::<Rc<T>>().ok().unwrap()
//             }
//             Self::SampleAt { x, logp } => {
//                 *x.downcast::<Rc<T>>().ok().unwrap()
//             }
//         }
//     }

//     pub fn from_sample<T>(v: T) -> Self {

//     }
// }

// impl<A: 'static,D: 'static,T: 'static> From<Trace<A,D,T>> for CallSite {
//     fn from(trace: Trace<A,D,T>) -> Self {
//         CallSite::TraceAt {
//             args: Rc::new(trace.args),
//             data: Rc::new(trace.data),
//             retv: match trace.retv {
//                 None => None,
//                 Some(v) => Some(Rc::new(v))
//             },
//             logp: trace.logp
//         }
//     }
// }

// impl<T: 'static> From<Sample<T>> for CallSite {
//     fn from(sample: Sample<T>) -> Self {
//         CallSite::SampleAt { x: Rc::new(sample.0), logp: 0. }
//     }
// }

// impl<A: 'static,D: 'static,T: 'static> Into<Rc<Trace<A,D,T>>> for CallSite {
//     fn into(self) -> Rc<Trace<A,D,T>> {
//         match self {
//             Self::TraceAt { args, data, retv, logp } => {
//                 Rc::new(Trace::new(
//                     *args.downcast::<A>().ok().unwrap(),
//                     *data.downcast::<D>().ok().unwrap(),
//                     *retv.unwrap().downcast::<T>().ok().unwrap(),
//                     logp
//                 ))
//             }
//             _ => {panic!("Expected CallSite::TraceAt")}
//         }
//     }
// }

// impl<T: 'static> Into<Rc<Sample<T>>> for CallSite {
//     fn into(self) -> Rc<Sample<T>> {
//         match self {
//             Self::SampleAt { x, logp } => {
//                 assert_eq!(logp, 0.);  // todo: is this correct?
//                 Rc::new(Sample(*x.downcast::<T>().ok().unwrap()))
//             }
//             _ => {panic!("Expected CallSite::SampleAt")}
//         }
//     }
// }