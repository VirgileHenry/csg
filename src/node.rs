
use crate::{
    bin_op::BinOp,
    primitive::Primitive,
    UnaryOp
};

#[derive(Debug, Clone, Copy)]
pub enum CsgNode {
    Primitive(Primitive),
    UnaryOp(UnaryOp),
    BinOp(BinOp),
}

impl CsgNode {
    pub fn id(&self) -> u32 {
        match self {
            CsgNode::Primitive(primitive) => primitive.id(),
            CsgNode::UnaryOp(un_op) => Primitive::VAR_COUNT + un_op.id(),
            CsgNode::BinOp(bin_op) => Primitive::VAR_COUNT + UnaryOp::VAR_COUNT + bin_op.id()
        }
    }
}