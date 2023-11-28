pub mod union;
pub mod substraction;
pub mod intersection;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::distance_func::DistanceFunc;
use self::union::CsgUnion;

/// Trait for any Csg operation object.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CsgOperation {
    Union(CsgUnion),
}

impl DistanceFunc for CsgOperation {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            CsgOperation::Union(union) => union.distance_function(at),
        }
    }
}

