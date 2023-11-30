use std::num::NonZeroUsize;

pub trait CsgTreeSize {
    fn size(&self) -> NonZeroUsize; 
}