use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    csg_object::Object,
    csg_binary_object::BinObject,
    csg_binary_operations::{
        BinOp, binary_intersection::BinInter,
        
    }, csg_traits::{distance_func::DistanceFunc, binarize::BinarizeCsgTree, csg_tree_size::CsgTreeSize, CsgTrait},
};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Inter {
    objects: Vec<Object>,
}

impl Inter {
    pub fn new(from: Vec<Object>) -> Self {
        Inter { objects: from }
    }
}

impl DistanceFunc for Inter {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        self.objects.iter().map(|o| o.distance_function(at))
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}

impl BinarizeCsgTree for Inter {
    fn binarize(self) -> Option<BinObject> {
        match self.objects.len() {
            0 => None, // op with no childs is an empty object
            1 => self.objects.into_iter().next().unwrap().binarize(),
            more => {
                let middle = more / 2;
                let mut second = self.objects;
                let first = second.drain(middle..).collect::<Vec<_>>();
                // consider the two parts as intersections, and binarize them
                let left = Inter::new(first).binarize();
                let right = Inter::new(second).binarize();
                match (left, right) {
                    (Some(left), Some(right)) => {
                        let op: BinOp = BinInter::new(left, right).into();
                        Some(op.into())
                    },
                    (Some(child), None) | (None, Some(child)) => Some(child),
                    (None, None) => None,
                }
            }
        }
    }
}

impl CsgTreeSize for Inter {
    fn size(&self) -> NonZeroUsize {
        let childs_size: usize = self.objects.iter().map(|o| o.size().get()).sum();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}

impl CsgTrait for Inter {}