

/// Trait for anything that have a distance func.
pub trait DistanceFunc {
    /// Get the distance from the point `at` to the object,
    /// considering it is at the origin and unrotated, unscaled.
    fn distance_function(&self, at: cgmath::Vector3<f32>) -> f32;
}

