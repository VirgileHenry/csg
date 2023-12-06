use crate::binary_object::BinObject;


/// The Binarize Csg Tree allow to build binary csg objects from regular csg objects.
/// As regular Csg are more convenient to work with, but binary ones are more deterministic,
/// this trait allow to easily go from one to another.
/// It is however a somewhat costly operation, as it moves data around and performs allocations.
pub trait BinarizeCsgTree {
    fn binarize(self) -> Option<BinObject>;
}