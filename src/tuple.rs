use core::ops;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tuple {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) w: f64,
}

impl Tuple {
    #[allow(dead_code)]
    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    #[allow(dead_code)]
    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple { x, y, z, w: 1.0 };
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        return Tuple { x, y, z, w: 0.0 };
    }

    pub fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        return Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        };
    }

    pub fn dot(&self, t0: Tuple) -> f64 {
        return self.x * t0.x + self.y * t0.y + self.z * t0.z + self.w * t0.w;
    }

    #[allow(dead_code)]
    pub fn cross(&self, t0: Tuple) -> Tuple {
        return Tuple::vector(
            self.y * t0.z - self.z * t0.y,
            self.z * t0.x - self.x * t0.z,
            self.x * t0.y - self.y * t0.x,
        );
    }

    #[allow(dead_code)]
    pub fn round(&self) -> Tuple {
        return Tuple {
            x: (self.x * 100000.0).round() / 100000.0,
            y: (self.y * 100000.0).round() / 100000.0,
            z: (self.z * 100000.0).round() / 100000.0,
            w: self.w,
        };
    }

    #[allow(dead_code)]
    pub fn reflect(self, normal: Tuple) -> Tuple {
        return self - normal * 2.0 * self.dot(normal);
    }
}

impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Out of bound index used {}", index),
        };
    }
}
impl IndexMut<usize> for Tuple {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Out of bound index used {}", index),
        };
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        return Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        };
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        return Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        };
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Self::Output {
        return Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        };
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        return Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        };
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        return Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let target = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(true, target.is_point());
    }

    #[test]
    fn tuple_with_w1_is_not_a_vector() {
        let target = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(false, target.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_not_a_point() {
        let target = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(false, target.is_point());
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let target = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(true, target.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w1() {
        let target = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(1.0, target.w)
    }

    #[test]
    fn vector_creates_tuple_with_w10() {
        let target = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(0.0, target.w)
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple {
            x: 1.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let result = t1 + t2;
        let expected = Tuple {
            x: -1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        let result = p1 - p2;
        let expected = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let result = p - v;

        let expected = Tuple::point(-2.0, -4.0, -6.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let result = v1 - v2;

        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        let result = zero - v;

        let expected = Tuple::vector(-1.0, 2.0, -3.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let result = -a;

        assert_eq!(
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            },
            result
        );
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };

        assert_eq!(expected, t * 3.5)
    }

    #[test]
    fn multiplying_tuple_by_fracture() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };

        assert_eq!(expected, t * 0.5)
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let t = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };

        assert_eq!(expected, t / 2.0)
    }

    macro_rules! magnitude_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let result = input.magnitude();

                assert_eq!(expected, result);
            }
        )*
        }
    }

    magnitude_tests! {
        computing_the_magnitude_of_vector_100: (Tuple::vector(1.0, 0.0, 0.0), 1.0),
        computing_the_magnitude_of_vector_010: (Tuple::vector(0.0, 1.0, 0.0), 1.0),
        computing_the_magnitude_of_vector_001: (Tuple::vector(0.0, 0.0, 1.0), 1.0),
        computing_the_magnitude_of_vector_123: (Tuple::vector(1.0, 2.0, 3.0), 14.0_f64.sqrt()),
        computing_the_magnitude_of_vector_neg123: (Tuple::vector(-1.0, -2.0, -3.0), 14.0_f64.sqrt()),
    }

    macro_rules! normalize_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let result = input.normalize();

                assert_eq!(expected, result);
            }
        )*
        }
    }

    normalize_tests! {
        normalizing_vector_400_returns_100: (Tuple::vector(4.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
        normalizing_vector_123:(Tuple::vector(1.0, 2.0, 3.0), Tuple::vector(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)),
    }

    #[test]
    fn magnitude_of_normalized_vector_equals_1() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let norm = v.normalize();

        assert_eq!(1.0, norm.magnitude());
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let result = a.dot(b);

        assert_eq!(20.0, result);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let result = a.cross(b);

        assert_eq!(Tuple::vector(-1.0, 2.0, -1.0), result);
    }

    #[test]
    fn the_cross_product_of_two_vectors_inv() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let result = b.cross(a);

        assert_eq!(Tuple::vector(1.0, -2.0, 1.0), result);
    }
}
