pub mod capped_torus;
pub mod capsule;
pub mod cube;
pub mod cylinder;
pub mod infinite_cylinder;
pub mod plane;
pub mod sphere;
pub mod torus;

use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{csg_traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgTrait}, csg_binary_object::BinObject};

use self::sphere::CsgSphere;

/// trait for any Csg primitive.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere(CsgSphere),
}

impl DistanceFunc for Primitive {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            Primitive::Sphere(sphere) => sphere.distance_function(at),
        }
    }
}

impl CsgTreeSize for Primitive {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for Primitive {
    fn binarize(self) -> Option<crate::csg_binary_object::BinObject> {
        Some(BinObject::Primitive(self))
    }
}

impl CsgTrait for Primitive {}

impl From<CsgSphere> for Primitive {
    fn from(value: CsgSphere) -> Self {
        Primitive::Sphere(value)
    }
}