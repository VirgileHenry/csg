
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Round {
        radius: crate::Float,
    }   
}

impl UnaryOp {
    pub(crate) const VAR_COUNT: u32 = 1;

    pub(crate) fn id(&self) -> u32 {
        match self {
            UnaryOp::Round { .. } => 0,
        }
    }
}