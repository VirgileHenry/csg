

/// Trait for anything that have a distance func.
/// The distance functions tells us what is the distance to the closest surface
/// of any Csg object, considered the object is at the origin.
pub trait DistanceFunc {
    /// Get the distance from the point `at` to the object,
    /// considering it is at the origin and unrotated, unscaled.
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32;
}

