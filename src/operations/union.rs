#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{distance_func::DistanceFunc, csg_object::CsgObject};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CsgUnion {
    objects: Vec<CsgObject>,
}

impl CsgUnion {
    pub fn new(from: Vec<CsgObject>) -> Self {
        CsgUnion { objects: from }
    }
}

impl DistanceFunc for CsgUnion {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.objects.iter().map(|o| o.distance_function(at))
            .fold(f32::INFINITY, |a, b| a.min(b))
    }
}