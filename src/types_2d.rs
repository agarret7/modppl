use serde::{Serialize, Deserialize};


pub struct Bounds { pub xmin: f32, pub xmax: f32, pub ymin: f32, pub ymax: f32 }

#[derive(Serialize, Deserialize, Debug)]
pub struct Point { pub x: f32, pub y: f32 }