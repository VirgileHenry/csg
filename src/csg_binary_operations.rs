pub mod binary_intersection;
pub mod binary_union;
pub mod substraction;

use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::csg_traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree};

use self::{substraction::Cut, binary_intersection::BinInter, binary_union::BinUnion};


#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum BinOp {
    Substraction(Cut),
    Union(BinUnion),
    Intersection(BinInter),
}

impl DistanceFunc for BinOp {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            BinOp::Substraction(sub) => sub.distance_function(at),
            BinOp::Intersection(int) => int.distance_function(at),
            BinOp::Union(uni) => uni.distance_function(at),
        }
    }
}

impl CsgTreeSize for BinOp {
    fn size(&self) -> NonZeroUsize {
        match self {
            BinOp::Substraction(sub) => sub.size(),
            BinOp::Intersection(int) => int.size(),
            BinOp::Union(uni) => uni.size(),
        }
    }
}

impl BinarizeCsgTree for BinOp {
    fn binarize(self) -> Option<crate::csg_binary_object::BinObject> {
        Some(self.into())
    }
}

impl From<BinInter> for BinOp {
    fn from(value: BinInter) -> Self {
        BinOp::Intersection(value)
    }
}

impl From<Cut> for BinOp {
    fn from(value: Cut) -> Self {
        BinOp::Substraction(value)
    }
}

impl From<BinUnion> for BinOp {
    fn from(value: BinUnion) -> Self {
        BinOp::Union(value)
    }
}