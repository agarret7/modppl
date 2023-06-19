use crate::{
    types_2d,
};


// think of this as a basic finite state machine
// capable of fast discrete integration
pub struct EnumerativeGrid {
    pub bounds: types_2d::Bounds,
    pub resolution_per_dimension: (u32, u32),
    scores: Vec<f32>,
    loglikelihood: Option<f32>
}

impl Iterator for EnumerativeGrid {
    type Item = types_2d::Point;

    fn next(&mut self) -> Option<types_2d::Point> {
        None
    }
}

impl EnumerativeGrid {
    pub fn new(
        center: &types_2d::Point,
        range_per_dimension: (f32, f32),
        resolution_per_dimension: (u32, u32)
    ) -> Self {
        let (xrange, yrange) = range_per_dimension;
        EnumerativeGrid {
            bounds: types_2d::Bounds {
                xmin: center.x - xrange/2., xmax: center.x + xrange/2.,
                ymin: center.y - yrange/2., ymax: center.y + yrange/2.
            },
            resolution_per_dimension: resolution_per_dimension,
            scores: vec![],
            loglikelihood: None
        }
    }

    // fn enumerate_over(&mut self, trace: &VecTrace) {
    //     self.loglikelihood = None;
    //     self.scores = vec![];
    //     for latent in self {
    //         let new_score = trace.update(latent).get_score();
    //     }
    // }

    fn normalize(&mut self) {
        match self.loglikelihood {
            Some(_) => { return },
            None => {
                self.loglikelihood = Some(0.);
                // normalize scores here
            }
        }
    }

    // fn sample_posterior(&self) -> Option<VecTrace> {
    //     match self.loglikelihood {
    //         Some(_) => {
    //             // sample some value from the posterior
    //             None
    //         }
    //         None => None
    //     }
    // }
}


// let mut grid = EnumerativeGrid::new(
//     trace.get_retval().last().unwrap(),
//     range_per_dimension,
//     resolution_per_dimension,
// );
// grid.enumerate_over(trace);
// grid.normalize();
// grid.sample_posterior()