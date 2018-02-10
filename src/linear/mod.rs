
//! Friendlier wrappers around the vecmath crate
mod vector;
pub use self::vector::{
    Vector2,
    Vector3,
    Vector4
};

mod vectori;
pub use self::vectori::{
    Vector2i,
    Vector3i,
    Vector4i
};

mod matrix;
pub use self::matrix::Matrix4;



// Conversion between vector types
impl From<Vector2i> for Vector2 {
    fn from(v: Vector2i) -> Self {
        Vector2 {
            x: v.x as f64,
            y: v.y as f64
        }
    }
}

impl From<Vector2> for Vector2i {
    fn from(v: Vector2) -> Self {
        Vector2i {
            x: v.x as i64,
            y: v.y as i64
        }
    }
}


impl From<Vector3i> for Vector3 {
    fn from(v: Vector3i) -> Self {
        Vector3 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}

impl From<Vector3> for Vector3i {
    fn from(v: Vector3) -> Self {
        Vector3i {
            x: v.x as i64,
            y: v.y as i64,
            z: v.z as i64,
        }
    }
}


impl From<Vector4i> for Vector4 {
    fn from(v: Vector4i) -> Self {
        Vector4 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
            w: v.w as f64,
        }
    }
}

impl From<Vector4> for Vector4i {
    fn from(v: Vector4) -> Self {
        Vector4i {
            x: v.x as i64,
            y: v.y as i64,
            z: v.z as i64,
            w: v.w as i64,
        }
    }
}