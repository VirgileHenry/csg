pub mod binary_intersection;
pub mod binary_union;
pub mod substraction;

use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::traits::{distance_func::DistanceFunc, tree_size::TreeSize, binarize::BinarizeCsgTree, CsgTrait, node_iter::NodeIter, CsgBinTrait, tree_height::TreeHeight, bounding_cube::BoundingCube};

use self::{substraction::Cut, binary_intersection::BinInter, binary_union::BinUnion};


#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum BinOp {
    Substraction(Cut),
    Union(BinUnion),
    Intersection(BinInter),
}

impl DistanceFunc for BinOp {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        match self {
            BinOp::Substraction(sub) => sub.distance_function(at),
            BinOp::Intersection(int) => int.distance_function(at),
            BinOp::Union(uni) => uni.distance_function(at),
        }
    }
}

impl TreeSize for BinOp {
    fn size(&self) -> NonZeroUsize {
        match self {
            BinOp::Substraction(sub) => sub.size(),
            BinOp::Intersection(int) => int.size(),
            BinOp::Union(uni) => uni.size(),
        }
    }
}

impl BinarizeCsgTree for BinOp {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        Some(self.into())
    }
}

impl NodeIter for BinOp {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        // I hate the fact that I have to collect them and into iter again,
        // but the compiler will shout that the opaque types are not the same.
        // hopefully the compiler will also optimize this away.
        match self {
            BinOp::Substraction(sub) => sub.nodes().collect::<Vec<_>>().into_iter(),
            BinOp::Intersection(int) => int.nodes().collect::<Vec<_>>().into_iter(),
            BinOp::Union(uni) => uni.nodes().collect::<Vec<_>>().into_iter(),
        }
    }
}

impl TreeHeight for BinOp {
    fn height(&self) -> NonZeroUsize {
        match self {
            BinOp::Intersection(inter) => inter.height(),
            BinOp::Substraction(sub) => sub.height(),
            BinOp::Union(uni) => uni.height(),
        }
    }
}

impl BoundingCube for BinOp {
    fn bounding_cube(&self) -> f32 {
        match self {
            BinOp::Intersection(inter) => inter.bounding_cube(),
            BinOp::Substraction(sub) => sub.bounding_cube(),
            BinOp::Union(uni) => uni.bounding_cube(),
        }
    }
}

impl CsgTrait for BinOp {}
impl CsgBinTrait for BinOp {}

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