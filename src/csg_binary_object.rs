
use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    csg_modifiers::Modifier,
    csg_binary_operations::BinOp,
    csg_primitives::Primitive, csg_traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgTrait},
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
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
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

impl CsgTrait for BinObject {}

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