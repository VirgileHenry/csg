use std::io::stdout;
use csg;

pub fn main() {
    let csg_tree = csg::csg!(
        csg::BinOp::Union => {
            csg::Primitive::sphere(3.0)
        } {
            csg::BinOp::Union => {
                csg::Primitive::sphere(1.0)
            } {
                csg::Primitive::sphere(2.0)
            }
        }
    );

    csg_tree.pretty_print(&mut stdout()).unwrap();
}