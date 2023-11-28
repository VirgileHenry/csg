#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{distance_func::DistanceFunc, csg_object::CsgObject};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CsgSubstraction {
    cutted: CsgObject,
    cutter: CsgObject,
}

impl CsgSubstraction {
    pub fn new(cutted: CsgObject, cutter: CsgObject) -> Self {
        CsgSubstraction {
            cutted, cutter,
        }
    }
}

impl DistanceFunc for CsgSubstraction {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.cutted.distance_function(at).min(-self.cutter.distance_function(at))
    }
}