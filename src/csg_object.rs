use std::num::NonZeroUsize;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::csg_binary_object::BinObject;
use crate::csg_binary_operations::BinOp;
use crate::csg_traits::CsgTrait;
use crate::csg_traits::binarize::BinarizeCsgTree;
use crate::csg_traits::csg_tree_size::CsgTreeSize;
use crate::csg_traits::distance_func::DistanceFunc;
use crate::csg_modifiers::Modifier;
use crate::csg_operations::Op;
use crate::csg_primitives::Primitive;

/// A Csg object can be any one of a primitive, operation, binary operation, modifier.
/// The entire object is a tree like structure.
/// Unlike the BinarizedCsgObject, this type offers greater user flexibility,
/// allowing easier construction.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Object {
    Primitive(Primitive),
    Operation(Op),
    BinaryOperation(BinOp),
    Modifier(Modifier<Object>),
}

impl DistanceFunc for Object {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            Object::Primitive(pr) => pr.distance_function(at),
            Object::Operation(op) => op.distance_function(at),
            Object::BinaryOperation(op) => op.distance_function(at),
            Object::Modifier(mo) => mo.distance_function(at), 
        }
    }
}

impl BinarizeCsgTree for Object {
    fn binarize(self) -> Option<BinObject> {
        match self {
            Object::Primitive(pr) => Some(BinObject::Primitive(pr)),
            Object::Operation(op) => op.binarize(),
            Object::BinaryOperation(op) => Some(BinObject::BinaryOperation(op)),
            Object::Modifier(mo) => mo.binarize(),
        }
    }
}

impl CsgTreeSize for Object {
    fn size(&self) -> NonZeroUsize {
        match self {
            Object::Primitive(_) => unsafe { NonZeroUsize::new_unchecked(1) },
            Object::Operation(op) => op.size(),
            Object::BinaryOperation(op) => op.size(),
            Object::Modifier(mo) => mo.size(),
        }
    }
}

impl CsgTrait for Object {}

impl From<Primitive> for Object {
    fn from(value: Primitive) -> Self {
        Object::Primitive(value)
    }
}

impl From<Op> for Object {
    fn from(value: Op) -> Self {
        Object::Operation(value)
    }
}

impl From<BinOp> for Object {
    fn from(value: BinOp) -> Self {
        Object::BinaryOperation(value)
    }
}

impl From<Modifier<Object>> for Object {
    fn from(value: Modifier<Object>) -> Self {
        Object::Modifier(value)
    }
}