
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Round {
        radius: crate::Float,
    }   
}