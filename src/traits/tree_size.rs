use std::num::NonZeroUsize;

pub trait TreeSize {
    fn size(&self) -> NonZeroUsize; 
}