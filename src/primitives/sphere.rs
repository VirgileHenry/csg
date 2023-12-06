#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{traits::{distance_func::DistanceFunc, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, node_iter::NodeIter, CsgTrait, CsgBinTrait}, node::Node};

use super::Primitive;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CsgSphere {
    center: glam::Vec3,
    radius: f32,
}

impl CsgSphere {
    pub fn centered(radius: f32) -> Self {
        CsgSphere {
            center: glam::Vec3::new(0., 0., 0.),
            radius,
        }
    }
}

impl DistanceFunc for CsgSphere {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        at.length() - self.radius
    }
}

impl CsgTreeSize for CsgSphere {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { std::num::NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for CsgSphere {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        let obj: Primitive = self.into();
        Some(obj.into())
    }
}

impl NodeIter for CsgSphere {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        std::iter::once(Node::PrimitiveSphere { center: self.center, radius: self.radius })
    }
}

impl CsgTrait for CsgSphere {}
impl CsgBinTrait for CsgSphere {}