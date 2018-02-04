extern crate vecmath;

mod linear;
pub use linear::*;

#[cfg(test)]
mod tests {
    use linear::*;


    #[test]
    fn vec2_eq() {
        assert_eq!(
            Vector2 { x: 2.0, y: 3.14 },
            Vector2 { x: 2.0, y: 3.14 }
        );
        assert_ne!(
            Vector2 { x: 1.5, y: 2.98 },
            Vector2 { x: 1.5, y: 3.78 }
        );
        assert_ne!(
            Vector2 { x: 5.2, y: 2.78 },
            Vector2 { x: 1.5, y: 2.78 }
        );
    }

    #[test]
    fn vec2_add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.5);

        let mut sum = a + b;
        assert!(sum.x == a.x + b.x && sum.y == a.y + b.y);

        sum += a;
        assert!(sum.x == a.x + a.x + b.x && sum.y == a.y + a.y + b.y);

        sum += b;
        assert!(sum.x == a.x + a.x + b.x + b.x && sum.y == a.y + a.y + b.y + b.y);
    }

    #[test]
    fn vec2_sub() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.5);

        let mut diff = a - b;
        assert!(diff.x == a.x - b.x && diff.y == a.y - b.y);

        diff -= a;
        assert!(diff.x == a.x - a.x - b.x && diff.y == a.y - a.y - b.y);

        diff -= b;
        assert!(diff.x == a.x - a.x - b.x - b.x && diff.y == a.y - a.y - b.y - b.y);
    }

    #[test]
    fn vec2_mul() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.5);

        let mut prod = a * b;
        assert!(prod.x == a.x * b.x && prod.y == a.y * b.y);

        prod *= a;
        assert!(prod.x == a.x * a.x * b.x && prod.y == a.y * a.y * b.y);

        prod *= b;
        assert!(prod.x == a.x * a.x * b.x * b.x && prod.y == a.y * a.y * b.y * b.y);

        assert_eq!(a * 4.0, Vector2::new(4.0, 8.0));
        assert_eq!(a * 3.0, 3.0 * a);
    }

    #[test]
    fn vec2_div() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.5);

        let mut quot = a / b;
        assert!(quot.x == a.x / b.x && quot.y == a.y / b.y);

        quot /= a;
        assert!(quot.x == a.x / a.x / b.x && quot.y == a.y / a.y / b.y);

        quot /= b;
        assert!(quot.x == a.x / a.x / b.x / b.x && quot.y == a.y / a.y / b.y / b.y);
    }

    #[test]
    fn matrix_translate() {
        let a = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let b = Vector3::new(1.0, 2.0, 3.0);

        let matrix_a = Matrix4::translated(Vector3::new(1.0, 0.0, 0.0));
        let matrix_b = matrix_a.translate(Vector3::new(0.0, 2.0, 3.0));

        let result = a * matrix_b;
        assert_eq!(result, a + Vector4::new(1.0, 2.0, 3.0, 0.0))
    }


    #[test]
    fn matrix_scale() {
        let a = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let b = Vector3::new(1.5, 2.0, 1.0);

        let matrix_a = Matrix4::scaled(b);
        let matrix_b = matrix_a.scale(Vector3::new(1.0, 1.0, 3.0));

        let result = a * matrix_b;
        println!("{:?}", result);
        assert_eq!(result, a * Vector4::new(1.5, 2.0, 3.0, 1.0))
    }
}
