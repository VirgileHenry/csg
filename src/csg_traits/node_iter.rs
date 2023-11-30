use crate::csg_node::Node;

/// An iterator over the nodes of the csg tree object.
/// The iterator guarantees to be in order, this is,
/// any node that has multiple children will put himself,
/// then the first child (fully expanded), then the next, etc.
/// 
/// So the following tree:
/// ```tree
/// Union
/// |- Inter
/// |   |- Sphere A
/// |   `- Sphere B
/// `- Sphere C
/// ```
/// Will throw an iterator of nodes: `Union, Inter, SphereA, SphereB, SphereC`
pub trait NodeIter {
    /// An iterator over the nodes of the csg tree object.
    /// The iterator guarantees to be in order, this is,
    /// any node that has multiple children will put himself,
    /// then the first child (fully expanded), then the next, etc.
    /// 
    /// So the following tree:
    /// ```tree
    /// Union
    /// |- Inter
    /// |   |- Sphere A
    /// |   `- Sphere B
    /// `- Sphere C
    /// ```
    /// Will throw an iterator of nodes: `Union, Inter, SphereA, SphereB, SphereC`
    fn nodes(&self) -> impl Iterator<Item = Node>;
}