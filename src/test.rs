

#[test]
fn create_csg() {
    // simulate real use outside this crate
    use crate as csg;
    
    let _empty_csg = csg::csg!(
        csg::BinOp::Union => {
            csg::BinOp::Union => {
                csg::Primitive::sphere(1.0)
            } {
                csg::Primitive::sphere(1.0)
            }
        } {
            csg::Primitive::sphere(1.0)
        }
    );
    
    let _weighted_csg = csg::csg!(
        csg::BinOp::Union, 1.0 => {
            csg::BinOp::Union, 2.5 => {
                csg::Primitive::sphere(1.0), 0.5
            } {
                csg::Primitive::sphere(1.0), 0.5
            }
        } {
            csg::Primitive::sphere(1.0), 2.0
        }
    );
    
}