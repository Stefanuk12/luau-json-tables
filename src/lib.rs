// Import each special type
mod color3;
mod vector2;
mod vector3;

// Then reexport them
pub use color3::Color3;
pub use vector2::Vector2;
pub use vector3::Vector3;

/// A special wrapper for each special data type.
#[derive(serde::Deserialize)]
pub struct SpecialWrapper<T> {
    data: T
}
