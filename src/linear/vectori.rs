pub use self::vector2i::Vector2i;
pub use self::vector3i::Vector3i;
pub use self::vector4i::Vector4i;

mod vector2i {
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
    pub struct Vector2i {
        pub x: i64,
        pub y: i64,
    }


    impl Vector2i {
        /// Create a new vector
        pub fn new(x: i64, y: i64) -> Vector2i {
            Vector2i { x, y }
        }


        /// Dot product
        pub fn dot(&self, other: &Vector2i) -> i64 {
            self.x * other.x + self.y * other.y
        }
    }


    // COMPARISON

    impl PartialEq<Vector2i> for Vector2i {
        fn eq(&self, other: &Vector2i) -> bool {
            self.x == other.x && self.y == other.y
        }

        fn ne(&self, other: &Vector2i) -> bool {
            self.x != other.x || self.y != other.y
        }
    }


    // ADDITION

    impl Add<Vector2i> for Vector2i {
        type Output = Vector2i;

        fn add(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl AddAssign<Vector2i> for Vector2i {
        fn add_assign(&mut self, rhs: Vector2i) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }


    // SUBTRACTION

    impl Sub<Vector2i> for Vector2i {
        type Output = Vector2i;

        fn sub(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl SubAssign<Vector2i> for Vector2i {
        fn sub_assign(&mut self, rhs: Vector2i) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector2i> for Vector2i {
        type Output = Vector2i;

        fn mul(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }

    impl Mul<i64> for Vector2i {
        type Output = Vector2i;

        fn mul(self, rhs: i64) -> Self::Output {
            Vector2i {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl Mul<Vector2i> for i64 {
        type Output = Vector2i;

        fn mul(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self * rhs.x,
                y: self * rhs.y,
            }
        }
    }

    impl MulAssign<Vector2i> for Vector2i {
        fn mul_assign(&mut self, rhs: Vector2i) {
            self.x *= rhs.x;
            self.y *= rhs.y;
        }
    }

    impl MulAssign<i64> for Vector2i {
        fn mul_assign(&mut self, rhs: i64) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector2i> for Vector2i {
        type Output = Vector2i;

        fn div(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }

    impl Div<i64> for Vector2i {
        type Output = Vector2i;

        fn div(self, rhs: i64) -> Self::Output {
            Vector2i {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }

    impl Div<Vector2i> for i64 {
        type Output = Vector2i;

        fn div(self, rhs: Vector2i) -> Self::Output {
            Vector2i {
                x: self / rhs.x,
                y: self / rhs.y,
            }
        }
    }

    impl DivAssign<Vector2i> for Vector2i {
        fn div_assign(&mut self, rhs: Vector2i) {
            self.x /= rhs.x;
            self.y /= rhs.y;
        }
    }


    // NEGATION

    impl Neg for Vector2i {
        type Output = Vector2i;

        fn neg(self) -> Self::Output {
            Vector2i { x: -self.x, y: -self.y }
        }
    }
}

mod vector3i {
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
    pub struct Vector3i {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }


    impl Vector3i {
        /// Create a new vector
        pub fn new(x: i64, y: i64, z: i64) -> Vector3i {
            Vector3i { x, y, z }
        }


        /// Dot product
        pub fn dot(&self, other: &Vector3i) -> i64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
    }


    // COMPARISON

    impl PartialEq<Vector3i> for Vector3i {
        fn eq(&self, other: &Vector3i) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }

        fn ne(&self, other: &Vector3i) -> bool {
            self.x != other.x || self.y != other.y || self.z != other.z
        }
    }


    // ADDITION

    impl Add<Vector3i> for Vector3i {
        type Output = Vector3i;

        fn add(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl AddAssign<Vector3i> for Vector3i {
        fn add_assign(&mut self, rhs: Vector3i) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }


    // SUBTRACTION

    impl Sub<Vector3i> for Vector3i {
        type Output = Vector3i;

        fn sub(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl SubAssign<Vector3i> for Vector3i {
        fn sub_assign(&mut self, rhs: Vector3i) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector3i> for Vector3i {
        type Output = Vector3i;

        fn mul(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl Mul<i64> for Vector3i {
        type Output = Vector3i;

        fn mul(self, rhs: i64) -> Self::Output {
            Vector3i {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    impl Mul<Vector3i> for i64 {
        type Output = Vector3i;

        fn mul(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
            }
        }
    }

    impl MulAssign<Vector3i> for Vector3i {
        fn mul_assign(&mut self, rhs: Vector3i) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
        }
    }

    impl MulAssign<i64> for Vector3i {
        fn mul_assign(&mut self, rhs: i64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector3i> for Vector3i {
        type Output = Vector3i;

        fn div(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
            }
        }
    }

    impl Div<i64> for Vector3i {
        type Output = Vector3i;

        fn div(self, rhs: i64) -> Self::Output {
            Vector3i {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    impl Div<Vector3i> for i64 {
        type Output = Vector3i;

        fn div(self, rhs: Vector3i) -> Self::Output {
            Vector3i {
                x: self / rhs.x,
                y: self / rhs.y,
                z: self / rhs.z,
            }
        }
    }

    impl DivAssign<Vector3i> for Vector3i {
        fn div_assign(&mut self, rhs: Vector3i) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
        }
    }


    // NEGATION

    impl Neg for Vector3i {
        type Output = Vector3i;

        fn neg(self) -> Self::Output {
            Vector3i { x: -self.x, y: -self.y, z: -self.z }
        }
    }
}

mod vector4i {
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
    pub struct Vector4i {
        pub x: i64,
        pub y: i64,
        pub z: i64,
        pub w: i64,
    }


    impl Vector4i {
        /// Create a new vector
        pub fn new(x: i64, y: i64, z: i64, w: i64) -> Vector4i {
            Vector4i { x, y, z, w }
        }

        /// Dot product
        pub fn dot(&self, other: &Vector4i) -> i64 {
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        }
    }


    // COMPARISON

    impl PartialEq<Vector4i> for Vector4i {
        fn eq(&self, other: &Vector4i) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
        }

        fn ne(&self, other: &Vector4i) -> bool {
            self.x != other.x || self.y != other.y || self.z != other.z || self.w != other.w
        }
    }


    // ADDITION

    impl Add<Vector4i> for Vector4i {
        type Output = Vector4i;

        fn add(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    impl AddAssign<Vector4i> for Vector4i {
        fn add_assign(&mut self, rhs: Vector4i) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
            self.w += rhs.w;
        }
    }


    // SUBTRACTION

    impl Sub<Vector4i> for Vector4i {
        type Output = Vector4i;

        fn sub(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w - rhs.w,
            }
        }
    }

    impl SubAssign<Vector4i> for Vector4i {
        fn sub_assign(&mut self, rhs: Vector4i) {
            self.x -= rhs.x;
            self.y -= rhs.y;
            self.z -= rhs.z;
            self.w -= rhs.w;
        }
    }


    // MULTIPLICATION

    impl Mul<Vector4i> for Vector4i {
        type Output = Vector4i;

        fn mul(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
                w: self.w * rhs.w,
            }
        }
    }

    impl Mul<i64> for Vector4i {
        type Output = Vector4i;

        fn mul(self, rhs: i64) -> Self::Output {
            Vector4i {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }

    impl Mul<Vector4i> for i64 {
        type Output = Vector4i;

        fn mul(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
                w: self * rhs.w,
            }
        }
    }

    impl MulAssign<Vector4i> for Vector4i {
        fn mul_assign(&mut self, rhs: Vector4i) {
            self.x *= rhs.x;
            self.y *= rhs.y;
            self.z *= rhs.z;
            self.w *= rhs.w;
        }
    }

    impl MulAssign<i64> for Vector4i {
        fn mul_assign(&mut self, rhs: i64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
            self.w *= rhs;
        }
    }


    // DIVISION

    impl Div<Vector4i> for Vector4i {
        type Output = Vector4i;

        fn div(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
                z: self.z / rhs.z,
                w: self.w / rhs.w,
            }
        }
    }

    impl Div<i64> for Vector4i {
        type Output = Vector4i;

        fn div(self, rhs: i64) -> Self::Output {
            Vector4i {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }

    impl Div<Vector4i> for i64 {
        type Output = Vector4i;

        fn div(self, rhs: Vector4i) -> Self::Output {
            Vector4i {
                x: self / rhs.x,
                y: self / rhs.y,
                z: self / rhs.z,
                w: self / rhs.w,
            }
        }
    }

    impl DivAssign<Vector4i> for Vector4i {
        fn div_assign(&mut self, rhs: Vector4i) {
            self.x /= rhs.x;
            self.y /= rhs.y;
            self.z /= rhs.z;
            self.w /= rhs.w;
        }
    }


    // NEGATION

    impl Neg for Vector4i {
        type Output = Vector4i;

        fn neg(self) -> Self::Output {
            Vector4i { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
        }
    }


    // CONVERSION
    impl From<[i64; 4]> for Vector4i {
        fn from(array: [i64; 4]) -> Self {
            Vector4i {
                x: array[0],
                y: array[1],
                z: array[2],
                w: array[3],
            }
        }
    }

    impl From<Vector4i> for [i64; 4] {
        fn from(vector: Vector4i) -> Self {
            [vector.x, vector.y, vector.z, vector.w]
        }
    }
}


impl From<Vector4i> for Vector3i {
    fn from(v: Vector4i) -> Self {
        Vector3i { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Vector4i> for Vector2i {
    fn from(v: Vector4i) -> Self {
        Vector2i { x: v.x, y: v.y }
    }
}

impl From<Vector3i> for Vector2i {
    fn from(v: Vector3i) -> Self {
        Vector2i { x: v.x, y: v.y }
    }
}
