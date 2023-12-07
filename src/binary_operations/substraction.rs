use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{traits::{distance_func::DistanceFunc, tree_size::TreeSize, binarize::BinarizeCsgTree, node_iter::NodeIter, CsgTrait, CsgBinTrait, bounding_cube::BoundingCube, tree_height::TreeHeight}, binary_object::BinObject, node::Node};

use super::BinOp;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Cut {
    cutted_cutter: Box<(BinObject, BinObject)>,
}

impl Cut {
    pub fn new(cutted: BinObject, cutter: BinObject) -> Self {
        Cut {
            cutted_cutter: Box::new((cutted, cutter)),
        }
    }
}

impl DistanceFunc for Cut {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        self.cutted_cutter.0.distance_function(at).min(-self.cutted_cutter.1.distance_function(at))
    }
}

impl TreeSize for Cut {
    fn size(&self) -> NonZeroUsize {
        let childs_size: usize = self.cutted_cutter.0.size().get() + self.cutted_cutter.1.size().get();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}

impl BinarizeCsgTree for Cut {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        let op: BinOp = self.into();
        Some(op.into())
    }
}

impl NodeIter for Cut {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        std::iter::once(Node::OpBinCut).chain(self.cutted_cutter.0.nodes()).chain(self.cutted_cutter.1.nodes())
    }
}

impl TreeHeight for Cut {
    fn height(&self) -> NonZeroUsize {
        self.cutted_cutter.0.height().max(self.cutted_cutter.1.height()).saturating_add(1)
    }
}

impl BoundingCube for Cut {
    fn bounding_cube(&self) -> f32 {
        self.cutted_cutter.0.bounding_cube()
    }
}

impl CsgTrait for Cut {}
impl CsgBinTrait for Cut {}