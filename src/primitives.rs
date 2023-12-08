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
use crate::{traits::{distance_func::DistanceFunc, tree_size::TreeSize, binarize::BinarizeCsgTree, CsgTrait, node_iter::NodeIter, tree_height::TreeHeight, bounding_cube::BoundingCube}, binary_object::BinObject};

use self::{sphere::Sphere, cube::Cube};

/// trait for any Csg primitive.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere(Sphere),
    Cube(Cube)
}

impl DistanceFunc for Primitive {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        match self {
            Primitive::Sphere(sphere) => sphere.distance_function(at),
            Primitive::Cube(cube) => cube.distance_function(at)
        }
    }
}

impl TreeSize for Primitive {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for Primitive {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        Some(BinObject::Primitive(self))
    }
}

impl NodeIter for Primitive {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        match self {
            Primitive::Sphere(sphere) => sphere.nodes().collect::<Vec<_>>().into_iter(),
            Primitive::Cube(cube) => cube.nodes().collect::<Vec<_>>().into_iter(),
        }
    }
}

impl TreeHeight for Primitive {
    fn height(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1) }
    }
}

impl BoundingCube for Primitive {
    fn bounding_cube(&self) -> f32 {
        match self {
            Primitive::Sphere(sphere) => sphere.bounding_cube(),
            Primitive::Cube(cube) => cube.bounding_cube(),
        }
    }
}

impl CsgTrait for Primitive {}

impl From<Sphere> for Primitive {
    fn from(value: Sphere) -> Self {
        Primitive::Sphere(value)
    }
}

impl From<Cube> for Primitive {
    fn from(value: Cube) -> Self {
        Primitive::Cube(value)
    }
}