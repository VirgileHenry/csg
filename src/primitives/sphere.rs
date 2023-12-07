use std::num::NonZeroUsize;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{traits::{distance_func::DistanceFunc, tree_size::TreeSize, binarize::BinarizeCsgTree, node_iter::NodeIter, CsgTrait, CsgBinTrait, tree_height::TreeHeight, bounding_cube::BoundingCube}, node::Node};

use super::Primitive;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Sphere {
    center: glam::Vec3,
    radius: f32,
}

impl Sphere {
    pub fn centered(radius: f32) -> Self {
        Sphere {
            center: glam::Vec3::new(0., 0., 0.),
            radius,
        }
    }

    pub fn at(self, at: glam::Vec3) -> Self {
        Sphere {
            center: at,
            ..self
        }
    }   
}

impl DistanceFunc for Sphere {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        at.length() - self.radius
    }
}

impl TreeSize for Sphere {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { std::num::NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for Sphere {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        let obj: Primitive = self.into();
        Some(obj.into())
    }
}

impl NodeIter for Sphere {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        std::iter::once(Node::PrimitiveSphere { center: self.center, radius: self.radius })
    }
}

impl TreeHeight for Sphere {
    fn height(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1) }
    }
}

impl BoundingCube for Sphere {
    fn bounding_cube(&self) -> f32 {
        self.radius + self.center.length()
    }
}


impl CsgTrait for Sphere {}
impl CsgBinTrait for Sphere {}