
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