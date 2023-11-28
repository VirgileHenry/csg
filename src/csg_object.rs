#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::distance_func::DistanceFunc;
use crate::operations::CsgOperation;
use crate::primitives::CsgPrimitive;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CsgObject {
    Primitive(CsgPrimitive),
    Operation(CsgOperation),
}

impl DistanceFunc for CsgObject {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            CsgObject::Primitive(primitive) => primitive.distance_function(at),
            CsgObject::Operation(operation) => operation.distance_function(at),
        }
    }
}