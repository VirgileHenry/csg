use cgmath::InnerSpace;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{csg_traits::{distance_func::DistanceFunc, CsgTrait, node_iter::NodeIter, csg_tree_size::CsgTreeSize, binarize::BinarizeCsgTree, CsgBinTrait}, csg_node::Node};

use super::Primitive;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CsgSphere {
    center: cgmath::Vector3<f32>,
    radius: f32,
}

impl CsgSphere {
    pub fn centered(radius: f32) -> Self {
        CsgSphere {
            center: cgmath::Vector3::<f32>::new(0., 0., 0.),
            radius,
        }
    }
}

impl DistanceFunc for CsgSphere {
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32 {
        at.magnitude() - self.radius
    }
}

impl CsgTreeSize for CsgSphere {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { std::num::NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for CsgSphere {
    fn binarize(self) -> Option<crate::csg_binary_object::BinObject> {
        let obj: Primitive = self.into();
        Some(obj.into())
    }
}

impl NodeIter for CsgSphere {
    fn nodes(&self) -> impl Iterator<Item = crate::csg_node::Node> {
        std::iter::once(Node::PrimitiveSphere { center: self.center, radius: self.radius })
    }
}

impl CsgTrait for CsgSphere {}
impl CsgBinTrait for CsgSphere {}