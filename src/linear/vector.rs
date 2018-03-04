pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;

mod vector2 {
    use std::cmp::PartialEq;
    use std::ops::{
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Neg,
    };

    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct Vector2 {
        pub x: f64,
        pub y: f64,
    }


    impl Vector2 {
        /// Create a new vector
        pub fn new(x: f64, y: f64) -> Vector2 {
            Vector2 { x, y }
        }


        /// Dot product
        pub fn dot(&self, other: Vector2) -> f64 {
            self.x * other.x + self.y * other.y
        }

        /// Length of vector
        pub fn len(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        /// Distance between vectors
        pub fn distance(&self, other: Vector2) -> f64 {
            (*self - other).len()
        }

        /// Normalized unit vector
        pub fn norm(&self) -> Vector2 {
            *self / self.len()
        }
    }


    // COMPARISON

    impl PartialEq<Vector2> for Vector2 {
        fn eq(&self, other: &Vector2) -> bool {
            self.x == other.x && self.y == other.y
        }

        fn ne(&self, other: &Vector2) -> bool {
            self.x != other.x || self.y != other.y
        }
    }


    // ADDITION

    impl Add<Vector2> for Vector2 {
        type Output = Vector2;

        fn add(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl AddAssign<Vector2> for Vector2 {
        fn add_assign(&mut self, rhs: Vector2) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }


    // SUBTRACTION

    impl Sub<Vector2> for Vector2 {
        type Output = Vector2;

        fn sub(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl SubAssign<Vector2> for Vector2 {
        fn sub_assign(&mut self, rhs: Vector2) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector2> for Vector2 {
        type Output = Vector2;

        fn mul(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl Mul<f64> for Vector2 {
        type Output = Vector2;

        fn mul(self, rhs: f64) -> Self::Output {
            Vector2 {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl Mul<Vector2> for f64 {
        type Output = Vector2;

        fn mul(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self * rhs.x,
                y: self * rhs.y,
            }
        }
    }

    impl MulAssign<Vector2> for Vector2 {
        fn mul_assign(&mut self, rhs: Vector2) {
            self.x *= rhs.x;
            self.y *= rhs.y;
        }
    }

    impl MulAssign<f64> for Vector2 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector2> for Vector2 {
        type Output = Vector2;

        fn div(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    impl Div<f64> for Vector2 {
        type Output = Vector2;

        fn div(self, rhs: f64) -> Self::Output {
            Vector2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl Div<Vector2> for f64 {
        type Output = Vector2;

        fn div(self, rhs: Vector2) -> Self::Output {
            Vector2 {
                x: self / rhs.x,
                y: self / rhs.y,
            }
        }
    }

    impl DivAssign<Vector2> for Vector2 {
        fn div_assign(&mut self, rhs: Vector2) {
            self.x /= rhs.x;
            self.y /= rhs.y;
        }
    }


    // NEGATION

    impl Neg for Vector2 {
        type Output = Vector2;

        fn neg(self) -> Self::Output {
            Vector2 { x: -self.x, y: -self.y }
        }
    }


    // CONVERSION
    impl From<[f64; 2]> for Vector2 {
        fn from(array: [f64; 2]) -> Self {
            Vector2 {
                x: array[0],
                y: array[1],
            }
        }
    }

    impl From<Vector2> for [f64; 2] {
        fn from(vector: Vector2) -> Self {
            [vector.x, vector.y]
        }
    }
}

mod vector3 {
    use std::cmp::PartialEq;
    use std::ops::{
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Neg,
    };

    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }


    impl Vector3 {
        /// Create a new vector
        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3 { x, y, z }
        }


        /// Dot product
        pub fn dot(&self, other: Vector3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        /// Length of vector
        pub fn len(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }

        /// Distance between vectors
        pub fn distance(&self, other: Vector3) -> f64 {
            (*self - other).len()
        }

        /// Normalized unit vector
        pub fn norm(&self) -> Vector3 {
            *self / self.len()
        }
    }


    // COMPARISON

    impl PartialEq<Vector3> for Vector3 {
        fn eq(&self, other: &Vector3) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }

        fn ne(&self, other: &Vector3) -> bool {
            self.x != other.x || self.y != other.y || self.z != other.z
        }
    }


    // ADDITION

    impl Add<Vector3> for Vector3 {
        type Output = Vector3;

        fn add(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl AddAssign<Vector3> for Vector3 {
        fn add_assign(&mut self, rhs: Vector3) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }


    // SUBTRACTION

    impl Sub<Vector3> for Vector3 {
        type Output = Vector3;

        fn sub(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl SubAssign<Vector3> for Vector3 {
        fn sub_assign(&mut self, rhs: Vector3) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector3> for Vector3 {
        type Output = Vector3;

        fn mul(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl Mul<f64> for Vector3 {
        type Output = Vector3;

        fn mul(self, rhs: f64) -> Self::Output {
            Vector3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl Mul<Vector3> for f64 {
        type Output = Vector3;

        fn mul(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
            }
        }
    }

    impl MulAssign<Vector3> for Vector3 {
        fn mul_assign(&mut self, rhs: Vector3) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }

    impl MulAssign<f64> for Vector3 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector3> for Vector3 {
        type Output = Vector3;

        fn div(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
            }
        }
    }

    impl Div<f64> for Vector3 {
        type Output = Vector3;

        fn div(self, rhs: f64) -> Self::Output {
            Vector3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    impl Div<Vector3> for f64 {
        type Output = Vector3;

        fn div(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self / rhs.x,
                y: self / rhs.y,
                z: self / rhs.z,
            }
        }
    }

    impl DivAssign<Vector3> for Vector3 {
        fn div_assign(&mut self, rhs: Vector3) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }


    // NEGATION

    impl Neg for Vector3 {
        type Output = Vector3;

        fn neg(self) -> Self::Output {
            Vector3 { x: -self.x, y: -self.y, z: -self.z }
        }
    }


    // CONVERSION
    impl From<[f64; 3]> for Vector3 {
        fn from(array: [f64; 3]) -> Self {
            Vector3 {
                x: array[0],
                y: array[1],
                z: array[2],
            }
        }
    }

    impl From<Vector3> for [f64; 3] {
        fn from(vector: Vector3) -> Self {
            [vector.x, vector.y, vector.z]
        }
    }
}

mod vector4 {
    use std::cmp::PartialEq;
    use std::ops::{
        Add, AddAssign,
        Sub, SubAssign,
        Mul, MulAssign,
        Div, DivAssign,
        Neg,
    };

    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct Vector4 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }


    impl Vector4 {
        /// Create a new vector
        pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
            Vector4 { x, y, z, w }
        }

        /// Dot product
        pub fn dot(&self, other: Vector4) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        }

        /// Length of vector
        pub fn len(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
        }

        /// Distance between vectors
        pub fn distance(&self, other: Vector4) -> f64 {
            (*self - other).len()
        }

        /// Normalized unit vector
        pub fn norm(&self) -> Vector4 {
            *self / self.len()
        }
    }


    // COMPARISON

    impl PartialEq<Vector4> for Vector4 {
        fn eq(&self, other: &Vector4) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
        }

        fn ne(&self, other: &Vector4) -> bool {
            self.x != other.x || self.y != other.y || self.z != other.z || self.w != other.w
        }
    }


    // ADDITION

    impl Add<Vector4> for Vector4 {
        type Output = Vector4;

        fn add(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    impl AddAssign<Vector4> for Vector4 {
        fn add_assign(&mut self, rhs: Vector4) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
            self.w += rhs.w;
        }
    }


    // SUBTRACTION

    impl Sub<Vector4> for Vector4 {
        type Output = Vector4;

        fn sub(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w - rhs.w,
            }
        }
    }

    impl SubAssign<Vector4> for Vector4 {
        fn sub_assign(&mut self, rhs: Vector4) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
            self.w -= rhs.w;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector4> for Vector4 {
        type Output = Vector4;

        fn mul(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
                w: self.w * rhs.w,
            }
        }
    }

    impl Mul<f64> for Vector4 {
        type Output = Vector4;

        fn mul(self, rhs: f64) -> Self::Output {
            Vector4 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }

    impl Mul<Vector4> for f64 {
        type Output = Vector4;

        fn mul(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
                w: self * rhs.w,
            }
        }
    }

    impl MulAssign<Vector4> for Vector4 {
        fn mul_assign(&mut self, rhs: Vector4) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
            self.w *= rhs.w;
        }
    }

    impl MulAssign<f64> for Vector4 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
            self.w *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector4> for Vector4 {
        type Output = Vector4;

        fn div(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
                w: self.w / rhs.w,
            }
        }
    }

    impl Div<f64> for Vector4 {
        type Output = Vector4;

        fn div(self, rhs: f64) -> Self::Output {
            Vector4 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }

    impl Div<Vector4> for f64 {
        type Output = Vector4;

        fn div(self, rhs: Vector4) -> Self::Output {
            Vector4 {
                x: self / rhs.x,
                y: self / rhs.y,
                z: self / rhs.z,
                w: self / rhs.w,
            }
        }
    }

    impl DivAssign<Vector4> for Vector4 {
        fn div_assign(&mut self, rhs: Vector4) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
            self.w /= rhs.w;
        }
    }


    // NEGATION

    impl Neg for Vector4 {
        type Output = Vector4;

        fn neg(self) -> Self::Output {
            Vector4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
        }
    }


    // CONVERSION
    impl From<[f64; 4]> for Vector4 {
        fn from(array: [f64; 4]) -> Self {
            Vector4 {
                x: array[0],
                y: array[1],
                z: array[2],
                w: array[3],
            }
        }
    }

    impl From<Vector4> for [f64; 4] {
        fn from(vector: Vector4) -> Self {
            [vector.x, vector.y, vector.z, vector.w]
        }
    }
}


impl From<Vector4> for Vector3 {
    fn from(v: Vector4) -> Self {
        Vector3 { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Vector4> for Vector2 {
    fn from(v: Vector4) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector3> for Vector2 {
    fn from(v: Vector3) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}
