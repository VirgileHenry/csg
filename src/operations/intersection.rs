#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{distance_func::DistanceFunc, csg_object::CsgObject};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CsgIntersection {
    objects: Vec<CsgObject>,
}

impl CsgIntersection {
    pub fn new(from: Vec<CsgObject>) -> Self {
        CsgIntersection { objects: from }
    }
}

impl DistanceFunc for CsgIntersection {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.objects.iter().map(|o| o.distance_function(at))
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}