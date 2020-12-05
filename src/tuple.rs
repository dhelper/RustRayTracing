use core::ops;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }
    pub fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        return Tuple {
            x,
            y,
            z,
            w: 1.0,
        };
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        return Tuple {
            x,
            y,
            z,
            w: 0.0,
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

impl std::ops::Neg for Tuple{
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

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use std::ops::{Add, Sub};

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
        let t1 = Tuple { x: 1.0, y: -2.0, z: 5.0, w: 1.0 };
        let t2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };

        let result = t1 + t2;
        let expected = Tuple { x: -1.0, y: 1.0, z: 6.0, w: 1.0 };

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
        let a = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };

        let result = -a;

        assert_eq!(Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 }, result);
    }
}