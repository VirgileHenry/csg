
/// A node is a compact representation of a binary op, modifier or primitive.
/// It only contains info about the current object, not on children.
/// Therefore, some nodes contain no info at all.
#[repr(u8)]
pub enum Node {
    // primitives (no following children)
    PrimitiveSphere {
        center: glam::Vec3,
        radius: f32,
    },
    PrimitiveCube {
        position: glam::Vec3,
        rotation: glam::Quat,
        scale: glam::Vec3,
    },

    // binary operations (two following children)
    OpBinCut,
    OpBinInter,
    OpBinUnion,

    // modifiers (one following child)
    ModRounder {
        radius: f32,
    },
}

impl Node {
    pub fn id(&self) -> u8 {
        match &self {
            Node::PrimitiveSphere { .. } => 1,
            Node::PrimitiveCube { .. } => 2,
            Node::OpBinCut => 20,
            Node::OpBinInter => 21,
            Node::OpBinUnion => 22,
            Node::ModRounder { .. } => 40,
        }
    }
}