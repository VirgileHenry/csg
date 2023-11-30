use self::{
    distance_func::DistanceFunc,
    binarize::BinarizeCsgTree,
    csg_tree_size::CsgTreeSize
};

pub mod binarize;
pub mod csg_tree_size;
pub mod distance_func;

pub trait CsgTrait: DistanceFunc + BinarizeCsgTree + CsgTreeSize {}