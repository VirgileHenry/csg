pub mod union;
pub mod intersection;

use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{binary_object::BinObject, traits::{distance_func::DistanceFunc, binarize::BinarizeCsgTree, tree_size::TreeSize, CsgTrait, tree_height::TreeHeight, bounding_cube::BoundingCube}};
use self::{union::Union, intersection::Inter};

/// Trait for any Csg operation object.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Op {
    Union(Union),
    Intersection(Inter),
}

impl DistanceFunc for Op {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        match self {
            Op::Union(union) => union.distance_function(at),
            Op::Intersection(inter) => inter.distance_function(at),
        }
    }
}

impl BinarizeCsgTree for Op {
    fn binarize(self) -> Option<BinObject> {
        match self {
            Op::Union(union) => union.binarize(),
            Op::Intersection(inter) => inter.binarize(),
        }
    }
}

impl TreeSize for Op {
    fn size(&self) -> NonZeroUsize {
        match self {
            Op::Union(union) => union.size(),
            Op::Intersection(inter) => inter.size(),
        }
    }
}

impl TreeHeight for Op {
    fn height(&self) -> NonZeroUsize {
        match self {
            Op::Union(union) => union.height(),
            Op::Intersection(inter) => inter.height(),
        }
    }
}

impl BoundingCube for Op {
    fn bounding_cube(&self) -> f32 {
        match self {
            Op::Union(union) => union.bounding_cube(),
            Op::Intersection(inter) => inter.bounding_cube(),
        }
    }
}

impl CsgTrait for Op {}

impl From<Union> for Op {
    fn from(value: Union) -> Self {
        Op::Union(value)
    }
}

impl From<Inter> for Op {
    fn from(value: Inter) -> Self {
        Op::Intersection(value)
    }
}