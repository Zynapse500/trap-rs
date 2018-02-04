
//! Friendlier wrappers around the vecmath crate
mod vector;
pub use self::vector::{
    Vector2,
    Vector3,
    Vector4
};

mod matrix;
pub use self::matrix::Matrix4;