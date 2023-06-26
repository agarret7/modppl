use crate::gfi::ChoiceVal;
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy)]
pub struct Bounds { pub xmin: f32, pub xmax: f32, pub ymin: f32, pub ymax: f32 }

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Point { pub x: f32, pub y: f32 }

impl ChoiceVal for Point { }