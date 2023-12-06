
use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    modifiers::Modifier,
    binary_operations::BinOp,
    primitives::Primitive, traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgTrait, node_iter::NodeIter, CsgBinTrait},
};


/// A binarized Csg Object is like a Csg Object,
/// without opened operations that have unknown amounts of children.
/// Compared to the CsgObject, this type is more predictible and easier to work with. 
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum BinObject {
    Primitive(Primitive),
    BinaryOperation(BinOp),
    Modifier(Modifier<BinObject>),
}

impl DistanceFunc for BinObject {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        match self {
            BinObject::Primitive(pr) => pr.distance_function(at),
            BinObject::BinaryOperation(op) => op.distance_function(at),
            BinObject::Modifier(mo) => mo.distance_function(at), 
        }
    }
}

impl CsgTreeSize for BinObject {
    fn size(&self) -> NonZeroUsize {
        match self {
            BinObject::Primitive(_) => unsafe { NonZeroUsize::new_unchecked(1) },
            BinObject::BinaryOperation(op) => op.size(),
            BinObject::Modifier(mo) => mo.size(),
        }
    }
}

impl BinarizeCsgTree for BinObject {
    fn binarize(self) -> Option<BinObject> {
        Some(self)
    }
}

impl NodeIter for BinObject {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        // I hate the fact that I have to collect them and into iter again,
        // but the compiler will shout that the opaque types are not the same.
        // hopefully the compiler will also optimize this away.
        match self {
            BinObject::Primitive(pr) => pr.nodes().collect::<Vec<_>>().into_iter(),
            BinObject::BinaryOperation(op) => op.nodes().collect::<Vec<_>>().into_iter(),
            BinObject::Modifier(mo) => mo.nodes().collect::<Vec<_>>().into_iter(),
        }
    }
}

impl CsgTrait for BinObject {}
impl CsgBinTrait for BinObject {}

impl From<Primitive> for BinObject {
    fn from(value: Primitive) -> Self {
        BinObject::Primitive(value)
    }
}

impl From<BinOp> for BinObject {
    fn from(value: BinOp) -> Self {
        BinObject::BinaryOperation(value)
    }
}

impl From<Modifier<BinObject>> for BinObject {
    fn from(value: Modifier<BinObject>) -> Self {
        BinObject::Modifier(value)
    }
}