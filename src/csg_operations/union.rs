use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    csg_object::Object,
    csg_binary_object::BinObject,
    csg_traits::{distance_func::DistanceFunc, binarize::BinarizeCsgTree, csg_tree_size::CsgTreeSize, CsgTrait}, csg_binary_operations::{BinOp, binary_union::BinUnion}
};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Union {
    objects: Vec<Object>,
}

impl Union {
    pub fn new(from: Vec<Object>) -> Self {
        Union { objects: from }
    }
}

impl DistanceFunc for Union {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.objects.iter().map(|o| o.distance_function(at))
            .fold(f32::INFINITY, |a, b| a.min(b))
    }
}

impl BinarizeCsgTree for Union {
    fn binarize(self) -> Option<BinObject> {
        match self.objects.len() {
            0 => None, // op with no childs is an empty object
            1 => self.objects.into_iter().next().unwrap().binarize(),
            more => {
                let middle = more / 2;
                let mut second = self.objects;
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
        let childs_size: usize = self.objects.iter().map(|o| o.size().get()).sum();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}

impl CsgTrait for Union {}