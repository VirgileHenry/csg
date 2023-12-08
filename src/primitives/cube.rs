use std::num::NonZeroUsize;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::{
    traits::{
        distance_func::DistanceFunc,
        tree_size::TreeSize,
        binarize::BinarizeCsgTree,
        node_iter::NodeIter,
        CsgTrait,
        CsgBinTrait,
        tree_height::TreeHeight,
        bounding_cube::BoundingCube
    },
    node::Node
};
use super::Primitive;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Cube {
    position: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

impl Cube {
    pub fn origin() -> Cube {
        Cube {
            position: glam::Vec3::ZERO,
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec3::ONE,
        }
    }

    pub fn at(self, at: glam::Vec3) -> Cube {
        Cube {
            position: at,
            ..self
        }
    }

    pub fn rotated(self, rot: glam::Quat) -> Cube {
        Cube {
            rotation: rot, 
            ..self
        }
    }

    pub fn scaled(self, scale: glam::Vec3) -> Cube {
        Cube {
            scale,
            ..self
        }
    }
}

impl DistanceFunc for Cube {
    fn distance_function(&self, at: glam::Vec3) -> f32 {
        // transform the point to 1x1x1 center aligned cube
        let at = at - self.position;
        let at = self.rotation * at;
        let at = at / self.scale;
        // now get the distance to the [1x1x1 cube]
        (at.x.abs() - 0.5).min(at.y.abs() - 0.5).min(at.y.abs() - 0.5)
    }
}

impl TreeSize for Cube {
    fn size(&self) -> std::num::NonZeroUsize {
        unsafe { std::num::NonZeroUsize::new_unchecked(1) }
    }
}

impl BinarizeCsgTree for Cube {
    fn binarize(self) -> Option<crate::binary_object::BinObject> {
        let obj: Primitive = self.into();
        Some(obj.into())
    }
}

impl NodeIter for Cube {
    fn nodes(&self) -> impl Iterator<Item = crate::node::Node> {
        std::iter::once(Node::PrimitiveCube { 
            position: self.position, 
            rotation: self.rotation,
            scale: self.scale,
        })
    }
}

impl TreeHeight for Cube {
    fn height(&self) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(1) }
    }
}

impl BoundingCube for Cube {
    fn bounding_cube(&self) -> f32 {
        self.position.length() + self.scale.length() * 0.5
    }
}


impl CsgTrait for Cube {}
impl CsgBinTrait for Cube {}