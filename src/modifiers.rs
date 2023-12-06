use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{binary_object::BinObject, traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgTrait, CsgBinTrait, node_iter::NodeIter}, node::Node};

/// Trait for any Csg operation object.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Modifier<T> {
    Rounding(Box<T>, f32)
}

impl<T: DistanceFunc> DistanceFunc for Modifier<T> {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        match self {
            Modifier::Rounding(obj, radius) => obj.distance_function(at) -  radius,
        }
    }
}

impl<T: CsgTreeSize> CsgTreeSize for Modifier<T> {
    fn size(&self) -> NonZeroUsize {
        match self {
            Modifier::Rounding(obj, _) => unsafe { NonZeroUsize::new_unchecked(1 + obj.size().get()) } 
        }
    }
}

impl<T: BinarizeCsgTree> BinarizeCsgTree for Modifier<T> {
    fn binarize(self) -> Option<BinObject> {
        match self {
            Self::Rounding(mo, radius) => Some(BinObject::Modifier(Modifier::Rounding(Box::new(mo.binarize()?), radius))),
        }
    }
}

impl<T: NodeIter> NodeIter for Modifier<T> {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        match self {
            Modifier::Rounding(obj, radius) => std::iter::once(Node::ModRounder { radius: *radius, }).chain(obj.nodes())
        }
    }
}

impl<T: CsgTrait> CsgTrait for Modifier<T> {}
impl<T: CsgBinTrait> CsgBinTrait for Modifier<T> {}