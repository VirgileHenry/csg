
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Union,
    Inter,
    Diff,
}

impl BinOp {
    pub(crate) fn id(&self) -> u32 {
        match self {
            BinOp::Union => 0,
            BinOp::Inter => 1,
            BinOp::Diff => 2,
        }
    }
}