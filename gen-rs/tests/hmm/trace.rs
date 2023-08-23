use gen_rs::Trace;


#[derive(Clone,Copy)]
pub struct ParamStore { }

pub type HMMTrace = Trace<(i64, ParamStore),(Vec<Option<usize>>,Vec<Option<usize>>),Vec<usize>>;

pub fn extend(trace: &mut HMMTrace, new_state: usize, new_observation: usize) -> () {
    trace.data.0.push(Some(new_state));
    trace.data.1.push(Some(new_observation));
    trace.args.0 += 1;
}

// pub fn validate(trace: &HMMTrace) -> () {
//     assert_eq!(trace.data.0.len() as i64, trace.args.0);
//     assert_eq!(trace.data.1.len() as i64, trace.args.0);
//     for state in trace.data.0.iter() {
//         assert!(state.as_ref() != None);
//     }
//     for obs in trace.data.1.iter() {
//         assert!(obs.as_ref() != None);
//     }
// }

// pub fn get_t(trace: &HMMTrace) -> i64 {
//     validate(trace);
//     trace.args.0
// }