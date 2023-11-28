pub mod capped_torus;
pub mod capsule;
pub mod cube;
pub mod cylinder;
pub mod infinite_cylinder;
pub mod plane;
pub mod sphere;
pub mod torus;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::distance_func::DistanceFunc;
use self::sphere::CsgSphere;

/// trait for any Csg primitive.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CsgPrimitive {
    Sphere(CsgSphere),
}

impl DistanceFunc for CsgPrimitive {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            CsgPrimitive::Sphere(sphere) => sphere.distance_function(at),
        }
    }
}

