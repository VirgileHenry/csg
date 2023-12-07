use std::num::NonZeroUsize;



/// Get the height of the tree.
pub trait TreeHeight {
    fn height(&self) -> NonZeroUsize;
}