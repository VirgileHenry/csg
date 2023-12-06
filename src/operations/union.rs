use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    object::Object,
    binary_object::BinObject,
    traits::{distance_func::DistanceFunc, binarize::BinarizeCsgTree, csg_tree_size::CsgTreeSize, CsgTrait}, binary_operations::{BinOp, binary_union::BinUnion}
};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Union {
    children: Vec<Object>,
}

impl Union {
    pub fn new(from: Vec<Object>) -> Self {
        Union { children: from }
    }
}

impl DistanceFunc for Union {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.children.iter().map(|o| o.distance_function(at))
            .fold(f32::INFINITY, |a, b| a.min(b))
    }
}

impl BinarizeCsgTree for Union {
    fn binarize(self) -> Option<BinObject> {
        match self.children.len() {
            0 => None, // op with no childs is an empty object
            1 => self.children.into_iter().next().unwrap().binarize(),
            more => {
                let middle = more / 2;
                let mut second = self.children;
                let first = second.drain(middle..).collect::<Vec<_>>();
                // consider the two parts as unions, and binarize them
                let left = Union::new(first).binarize();
                let right = Union::new(second).binarize();
                match (left, right) {
                    (Some(left), Some(right)) => {
                        let op: BinOp = BinUnion::new(left, right).into();
                        Some(op.into())
                    },
                    (Some(child), None) | (None, Some(child)) => Some(child),
                    (None, None) => None,
                }
            }
        }
    }
}

impl CsgTreeSize for Union {
    fn size(&self) -> NonZeroUsize {
        let childs_size: usize = self.children.iter().map(|o| o.size().get()).sum();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}


impl CsgTrait for Union {}