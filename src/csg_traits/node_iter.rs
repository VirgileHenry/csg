use crate::csg_node::Node;


pub trait NodeIter {
    fn nodes(&self) -> impl Iterator<Item = Node>;
}