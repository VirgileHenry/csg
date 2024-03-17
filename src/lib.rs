//* Core idea is to store csg in vecs, contiguous memory
//* We store the csg tree in prefixed notation: so Union Cube Inter Sphere Cube for example

pub mod bin_op;
pub mod node;
pub mod primitive;
pub mod unary_op;

#[cfg(test)]
mod test;

pub use {
    primitive::Primitive,
    bin_op::BinOp,
    unary_op::UnaryOp,
};

#[cfg(not(feature = "double_precision_floats"))]
type Float = f32;
#[cfg(not(feature = "double_precision_floats"))]
type Vec3 = glam::Vec3;
#[cfg(feature = "double_precision_floats")]
type Float = f64;
#[cfg(feature = "double_precision_floats")]
type Vec3 = glam::DVec3;

/// Csg object, not loaded so just the tree.
pub type CSG = LoadedCSG<()>;

/// Csg tree object that can add an element of type T to all nodes.
pub struct LoadedCSG<T> {
    /// SAFETY: this vec must respect the prefix notation when built
    csg_tree: Vec<(node::CsgNode, T)>,
}

impl<T> LoadedCSG<T> {
    pub fn empty() -> LoadedCSG<T> {
        LoadedCSG { 
            csg_tree: Vec::with_capacity(0),
        }
    }
    pub unsafe fn new_unchecked(tree: Vec<(node::CsgNode, T)>) -> LoadedCSG<T> {
        LoadedCSG {
            csg_tree: tree,
        }
    }
    pub fn node_count(&self) -> usize {
        self.csg_tree.len()
    }
    pub fn height(&self) -> usize {
        unimplemented!()
    }
    pub fn nodes(&self) -> impl Iterator<Item = &node::CsgNode> {
        self.csg_tree.iter().map(|(node, _)| node)
    }
    pub fn payloads(&self) -> impl Iterator<Item = &T> {
        self.csg_tree.iter().map(|(_, payload)| payload)
    }
    pub fn pretty_print<W: std::io::Write>(&self, output: &mut W) -> Result<(), std::io::Error> {
        for (node, _) in self  .csg_tree.iter() {
            write!(output, "{node:?}\n")?;
        }
        Ok(())
    } 
}


#[macro_export]
macro_rules! csg {
    // ## Compute capacity of csg tree
    // ### Primitives 
    (@capacity $primitive:expr ) => { csg::csg!(@capacity $primitive, ()) };
    (@capacity $primitive:expr, $payload:expr ) => { 1 };
    // ### Binary operations
    (@capacity $bin_op:expr => { $($child_1:tt)* } { $($child_2:tt)* } ) => {
        csg::csg!(@capacity $bin_op, () => { $($child_1)* } { $($child_2)* })
    };
    (@capacity $bin_op:expr, $payload:expr => { $($child_1:tt)* } { $($child_2:tt)* } ) => {
        1 + csg::csg!(@capacity $($child_1)* ) + csg::csg!(@capacity $($child_2)* )
    };
    // ## Build the csg tree into the vec
    // ### Primitives
    (@building $tree_vec:ident $primitive:expr ) => { csg::csg!(@building $tree_vec $primitive, ()) };
    (@building $tree_vec:ident $primitive:expr, $payload:expr ) => {
        $tree_vec.push((csg::node::CsgNode::Primitive($primitive), $payload));
    };
    // ### Binary operations
    (@building $tree_vec:ident $bin_op:expr => { $($child_1:tt)* } { $($child_2:tt)* } ) => {
        csg::csg!(@building $tree_vec $bin_op, () => { $($child_1)* } { $($child_2)* } )
    };
    (@building $tree_vec:ident $bin_op:expr, $payload:expr => { $($child_1:tt)* } { $($child_2:tt)* } ) => {
        $tree_vec.push((csg::node::CsgNode::BinOp($bin_op), $payload));
        csg::csg!(@building $tree_vec $($child_1)*);
        csg::csg!(@building $tree_vec $($child_2)*);
    };
    // ## Default case, init
    ($($tokens:tt)*) => {
        {
            let mut csg_tree = Vec::with_capacity(csg::csg!(@capacity $($tokens)*));
            csg::csg!(@building csg_tree $($tokens)*);
            unsafe { csg::LoadedCSG::new_unchecked(csg_tree) }
        }
    };
}
