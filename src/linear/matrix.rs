pub use self::matrix4::Matrix4;

mod matrix4 {
    use std::ops::Mul;
    use super::super::{
        Vector3,
        Vector4,
    };
    use vecmath::{
        col_mat4_mul,
        mat4_id,
        col_mat4_transform,
    };

    #[derive(Copy, Clone)]
    pub struct Matrix4 {
        data: [[f64; 4]; 4]
    }

    impl Matrix4 {
        pub fn new() -> Matrix4 {
            mat4_id().into()
        }

        pub fn translated(amount: Vector3) -> Matrix4 {
            Matrix4 {
                data: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [amount.x, amount.y, amount.z, 1.0],
                ],
            }
        }

        pub fn scaled(amount: Vector3) -> Matrix4 {
            Matrix4 {
                data: [
                    [amount.x, 0.0, 0.0, 0.0],
                    [0.0, amount.y, 0.0, 0.0],
                    [0.0, 0.0, amount.z, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            }
        }


        pub fn orthographic(left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64) -> Matrix4 {
            Matrix4::translated(Vector3::new(
                -(left+right)/2.0, -(top+bottom)/2.0, -(far+near)/2.0
            )).scale(Vector3::new(
                2.0 / (right - left), 2.0 / (top-bottom), 2.0 / (far - near)
            ))
        }


        pub fn translate(self, amount: Vector3) -> Matrix4 {
            let translation = Matrix4::translated(amount);
            self * translation
        }

        pub fn scale(self, amount: Vector3) -> Matrix4 {
            let scaling = Matrix4::scaled(amount);
            self * scaling
        }
    }


    // Matrix-Matrix multiplication
    impl Mul<Matrix4> for Matrix4 {
        type Output = Matrix4;

        fn mul(self, rhs: Matrix4) -> Self::Output {
            col_mat4_mul(self.data, rhs.data).into()
        }
    }


    // Matrix-Vector multiplication
    impl Mul<Matrix4> for Vector4 {
        type Output = Vector4;

        fn mul(self, rhs: Matrix4) -> Self::Output {
            col_mat4_transform(rhs.into(), self.into()).into()
        }
    }


    impl From<[[f64; 4]; 4]> for Matrix4 {
        fn from(data: [[f64; 4]; 4]) -> Self {
            Matrix4 { data }
        }
    }

    impl From<Matrix4> for [[f64; 4]; 4] {
        fn from(matrix: Matrix4) -> Self {
            matrix.data
        }
    }

    impl From<Matrix4> for [[f32; 4]; 4] {
        fn from(matrix: Matrix4) -> Self {
            [
                [matrix.data[0][0] as f32, matrix.data[0][1] as f32, matrix.data[0][2] as f32, matrix.data[0][3] as f32],
                [matrix.data[1][0] as f32, matrix.data[1][1] as f32, matrix.data[1][2] as f32, matrix.data[1][3] as f32],
                [matrix.data[2][0] as f32, matrix.data[2][1] as f32, matrix.data[2][2] as f32, matrix.data[2][3] as f32],
                [matrix.data[3][0] as f32, matrix.data[3][1] as f32, matrix.data[3][2] as f32, matrix.data[3][3] as f32],
            ]
        }
    }
}
