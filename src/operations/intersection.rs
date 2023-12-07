use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    object::Object,
    binary_object::BinObject,
    binary_operations::{
        BinOp, binary_intersection::BinInter,
        
    }, traits::{distance_func::DistanceFunc, binarize::BinarizeCsgTree, tree_size::TreeSize, CsgTrait, tree_height::TreeHeight, bounding_cube::BoundingCube},
};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Inter {
    children: Vec<Object>,
}

impl Inter {
    pub fn new(from: Vec<Object>) -> Self {
        Inter { children: from }
    }
}

impl DistanceFunc for Inter {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        self.children.iter().map(|o| o.distance_function(at))
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}

impl BinarizeCsgTree for Inter {
    fn binarize(self) -> Option<BinObject> {
        match self.children.len() {
            0 => None, // op with no childs is an empty object
            1 => self.children.into_iter().next().unwrap().binarize(),
            more => {
                let middle = more / 2;
                let mut second = self.children;
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

impl TreeSize for Inter {
    fn size(&self) -> NonZeroUsize {
        let childs_size: usize = self.children.iter().map(|o| o.size().get()).sum();
        unsafe { NonZeroUsize::new_unchecked(1 + childs_size) }
    }
}

impl TreeHeight for Inter {
    fn height(&self) -> NonZeroUsize {
        self.children.iter().map(|o| o.height())
            .fold(unsafe { NonZeroUsize::new_unchecked(1) }, |a, b| a.max(b))
    }
}

impl BoundingCube for Inter {
    fn bounding_cube(&self) -> f32 {
        self.children.iter().map(|o| o.bounding_cube())
            .fold(f32::INFINITY, |a, b| a.min(b))
    }
}



impl CsgTrait for Inter {}