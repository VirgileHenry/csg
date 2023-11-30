use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{csg_object::Object, csg_traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree}};

use super::BinOp;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Cut {
    cutted_cutter: Box<(Object, Object)>,
}

impl Cut {
    pub fn new(cutted: Object, cutter: Object) -> Self {
        Cut {
            cutted_cutter: Box::new((cutted, cutter)),
        }
    }
}

impl DistanceFunc for Cut {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.cutted_cutter.0.distance_function(at).min(-self.cutted_cutter.1.distance_function(at))
    }
}

impl CsgTreeSize for Cut {
    fn size(&self) -> NonZeroUsize {
        let childs_size: usize = self.cutted_cutter.0.size().get() + self.cutted_cutter.1.size().get();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}

impl BinarizeCsgTree for Cut {
    fn binarize(self) -> Option<crate::csg_binary_object::BinObject> {
        let op: BinOp = self.into();
        Some(op.into())
    }
}