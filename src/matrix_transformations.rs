use crate::matrix::Matrix4;

impl Matrix4 {
    pub fn inverse(self) -> Matrix4 {
        let mut result: [[f64; 4]; 4] = Default::default();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }

        return Matrix4::new(result);
    }

    #[allow(dead_code)]
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        return Matrix4::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }

    #[allow(dead_code)]
    pub fn translate(self, x: f64, y: f64, z: f64) -> Matrix4 {
        return Matrix4::translation(x, y, z) * self;
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
        return Matrix4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Matrix4 {
        return Matrix4::scaling(x, y, z) * self;
    }

    #[allow(dead_code)]
    pub fn rotation_x(r: f64) -> Matrix4 {
        return Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }

    #[allow(dead_code)]
    pub fn rotate_x(self, r: f64) -> Matrix4 {
        return Matrix4::rotation_x(r) * self;
    }

    #[allow(dead_code)]
    pub fn rotation_y(r: f64) -> Matrix4 {
        return Matrix4::new([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }

    #[allow(dead_code)]
    pub fn rotate_y(self, r: f64) -> Matrix4 {
        return Matrix4::rotation_y(r) * self;
    }

    pub fn rotation_z(r: f64) -> Matrix4 {
        return Matrix4::new([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }

    pub fn rotate_z(self, r: f64) -> Matrix4 {
        return Matrix4::rotation_z(r) * self;
    }

    #[allow(dead_code)]
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
        return Matrix4::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let m = Matrix4::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0]
        ]);

        let result = m.inverse();

        let expected = Matrix4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]
        ]);

        let actual = result.round();
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let m = Matrix4::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0]
        ]);

        let result = m.inverse();

        let expected = Matrix4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]
        ]);

        let actual = result.round();
        assert_eq!(expected, actual);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let m = Matrix4::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0]
        ]);

        let result = m.inverse();

        let expected = Matrix4::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]
        ]);

        let actual = result.round();
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix4::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0]
        ]);

        let m2 = Matrix4::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0]
        ]);

        let multiply_result = m1 * m2;

        let result = multiply_result * m2.inverse();

        assert_eq!(m1, result.round());
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        let result = transform * p;

        let expected = Tuple::point(2.0, 1.0, 7.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        let result = inv * p;

        let expected = Tuple::point(-8.0, 7.0, 3.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn translation_does_not_effect_vectors() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        let result = transform * v;

        assert_eq!(v, result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        let result = transform * p;
        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let result = transform * v;
        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let result = inv * v;
        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let result = transform * p;
        let expected = Tuple::point(-2.0, 3.0, 4.0);

        assert_eq!(expected, result);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis_half_quarter() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI / 4.0);

        let result = half_quarter * p;
        let expected = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn rotating_a_point_around_the_x_axis_full_quarter() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let full_quarter = Matrix4::rotation_x(PI / 2.0);

        let result = full_quarter * p;
        let expected = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();

        let result = inv * p;
        let expected = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn rotating_a_point_around_the_y_axis_half_quarter() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(PI / 4.0);

        let result = half_quarter * p;
        let expected = Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn rotating_a_point_around_the_y_axis_full_quarter() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let full_quarter = Matrix4::rotation_y(PI / 2.0);

        let result = full_quarter * p;
        let expected = Tuple::point(1.0, 0.0, 0.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn rotating_a_point_around_the_z_axis_half_quarter() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(PI / 4.0);

        let result = half_quarter * p;
        let expected = Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        assert_eq!(expected.round(), result.round());
    }

    #[test]
    fn rotating_a_point_around_the_z_axis_full_quarter() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let full_quarter = Matrix4::rotation_z(PI / 2.0);

        let result = full_quarter * p;
        let expected = Tuple::point(-1.0, 0.0, 0.0);

        assert_eq!(expected.round(), result.round());
    }

    macro_rules! shearing_tests {
        ($($name:ident: $value:expr,)*) => {
             $(
            #[test]
            fn $name(){
                let (xy, xz, yx, yz, zx, zy, p, expected) = $value;
                let transform = Matrix4::shearing(xy, xz, yx, yz, zx, zy);

                let result = transform * p;

                assert_eq!(expected, result);
            }
            )*
        }
    }

    shearing_tests! {
        a_shearing_transformation_moves_x_in_proportion_to_y: (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(5.0,3.0, 4.0)),
        a_shearing_transformation_moves_x_in_proportion_to_z: (0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(6.0,3.0, 4.0)),
        a_shearing_transformation_moves_y_in_proportion_to_x: (0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(2.0,5.0, 4.0)),
        a_shearing_transformation_moves_y_in_proportion_to_z: (0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(2.0,7.0, 4.0)),
        a_shearing_transformation_moves_z_in_proportion_to_x: (0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(2.0,3.0, 6.0)),
        a_shearing_transformation_moves_z_in_proportion_to_y: (0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        Tuple::point(2.0,3.0,4.0), Tuple::point(2.0,3.0, 7.0)),
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);

        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(Tuple::point(1.0, -1.0, 0.0), p2.round());

        let p3 = b * p2;
        assert_eq!(Tuple::point(5.0, -5.0, 0.0), p3.round());

        let p4 = c * p3;
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), p4.round());
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);

        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        let result = t * p;
        assert_eq!(Tuple::point(15.0, 0.0, 7.0), result.round());
    }

    #[test]
    fn chained_fluent_transformations_are_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);

        // need to apply in reverse order
        let t = Matrix4::identity()
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        let result = t * p;
        assert_eq!(Tuple::point(10.0, 0.0, 2.0), result.round());
    }
}