use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    binary_object::BinObject,
    traits::{distance_func::DistanceFunc, tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgTrait, node_iter::NodeIter, CsgBinTrait}, node::Node
};

use super::BinOp;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinInter {
    children: Box<(BinObject, BinObject)>,
}

impl BinInter {
    pub fn new(left: BinObject, right: BinObject) -> Self {
        BinInter { children: Box::new((left, right)) }
    }
}

impl DistanceFunc for BinInter {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        self.children.0.distance_function(at).max(self.children.1.distance_function(at))
    }
}

impl CsgTreeSize for BinInter {
    fn size(&self) -> NonZeroUsize {
        unsafe {
            NonZeroUsize::new_unchecked(
                1 + self.children.0.size().get() + self.children.1.size().get()
            )
        }
    }
}

impl BinarizeCsgTree for BinInter {
    fn binarize(self) -> Option<BinObject> {
        let op: BinOp = self.into();
        Some(op.into())
    }
}

impl NodeIter for BinInter {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        std::iter::once(Node::OpBinInter).chain(self.children.0.nodes()).chain(self.children.1.nodes())
    }
}

impl CsgTrait for BinInter {}
impl CsgBinTrait for BinInter {}