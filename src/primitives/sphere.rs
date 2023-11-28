use cgmath::InnerSpace;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
use crate::distance_func::DistanceFunc;

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
