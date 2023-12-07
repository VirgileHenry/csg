use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{binary_object::BinObject, traits::{distance_func::DistanceFunc, tree_size::TreeSize, binarize::BinarizeCsgTree, CsgTrait, CsgBinTrait, node_iter::NodeIter, tree_height::TreeHeight, bounding_cube::BoundingCube}, node::Node};

/// Trait for any Csg operation object.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Modifier<T> {
    Rounding(Box<T>, f32)
}

impl<T: DistanceFunc> DistanceFunc for Modifier<T> {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        match self {
            Modifier::Rounding(obj, radius) => obj.distance_function(at) -  radius,
        }
    }
}

impl<T: TreeSize> TreeSize for Modifier<T> {
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

impl<T: CsgTrait> TreeHeight for Modifier<T> {
    fn height(&self) -> NonZeroUsize {
        match self {
            Modifier::Rounding(obj, _) => obj.height().saturating_add(1),
        }
    }
}

impl<T: CsgTrait> BoundingCube for Modifier<T> {
    fn bounding_cube(&self) -> f32 {
        match self {
            Modifier::Rounding(obj, r) => obj.bounding_cube() + r,
        }
    }
}
impl<T: CsgTrait> CsgTrait for Modifier<T> {}
impl<T: CsgBinTrait> CsgBinTrait for Modifier<T> {}