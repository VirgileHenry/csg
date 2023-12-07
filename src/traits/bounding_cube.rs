
/// get the max amount a csg object will extend from the center.
/// This is half the AABB bounding box side for this object.
pub trait BoundingCube {
    fn bounding_cube(&self) -> f32;
}