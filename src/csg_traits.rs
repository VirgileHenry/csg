use self::{
    distance_func::DistanceFunc,
    binarize::BinarizeCsgTree,
    csg_tree_size::CsgTreeSize,
    node_iter::NodeIter
};

pub mod binarize;
pub mod csg_tree_size;
pub mod distance_func;
pub mod node_iter;

/// Csg Trait. regroups all traits that the Csg Have.
pub trait CsgTrait: DistanceFunc + BinarizeCsgTree + CsgTreeSize {}
pub trait CsgBinTrait: CsgTrait + NodeIter {}