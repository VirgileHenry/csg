pub mod binarize;
pub mod tree_size;
pub mod distance_func;
pub mod node_iter;
pub mod tree_height;
pub mod bounding_cube;

use self::{
    distance_func::DistanceFunc,
    binarize::BinarizeCsgTree,
    tree_size::TreeSize,
    node_iter::NodeIter,
    tree_height::TreeHeight,
    bounding_cube::BoundingCube
};

/// Csg Trait. regroups all traits that the Csg Have.
pub trait CsgTrait: DistanceFunc + BinarizeCsgTree + TreeSize + TreeHeight + BoundingCube {}
pub trait CsgBinTrait: CsgTrait + NodeIter {}