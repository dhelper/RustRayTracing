use std::ops::Index;

#[macro_export]
macro_rules! matrix {
    ($name:ident, $n:expr) =>
    {
        #[derive(Debug)]
        #[derive(PartialEq)]
        pub struct $name {
            values: [[f64; $n]; $n]
        }

        impl Index<(usize, usize)> for $name {
            type Output = f64;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                &self.values[index.0][index.1]
            }
        }
    }
}

matrix!(Matrix4, 4);
matrix!(Matrix3, 3);
matrix!(Matrix2, 2);

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix2;

    #[test]
    fn constructing_an_inspecting_a_4x4_matrix() {
        let m = Matrix4
        {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        };

        assert_eq!(1.0, m[(0, 0)]);
        assert_eq!(4.0, m[(0, 3)]);
        assert_eq!(5.5, m[(1, 0)]);
        assert_eq!(7.5, m[(1, 2)]);
        assert_eq!(11.0, m[(2, 2)]);
        assert_eq!(13.5, m[(3, 0)]);
        assert_eq!(15.5, m[(3, 2)]);
    }

    #[test]
    fn a_2x2_matrix_out_to_be_representable() {
        let m = Matrix2 {
            values: [
                [-3.0, 5.0],
                [1.0, -2.0]
            ]
        };

        assert_eq!(-3.0, m[(0, 0)]);
        assert_eq!(5.0, m[(0, 1)]);
        assert_eq!(1.0, m[(1, 0)]);
        assert_eq!(-2.0, m[(1, 1)]);
    }

    #[test]
    fn a_3x3_matrix_out_to_be_representable() {
        let m = Matrix3 {
            values: [
                [-3.0, 5.0, 0.0],
                [1.0, -2.0, -7.0],
                [0.0, 1.0, 1.0],
            ]
        };

        assert_eq!(-3.0, m[(0, 0)]);
        assert_eq!(-2.0, m[(1, 1)]);
        assert_eq!(1.0, m[(2, 2)]);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let m1 = Matrix4
        {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        };

        let m2 = Matrix4
        {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        };

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let m1 = Matrix4
        {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        };

        let m2 = Matrix4
        {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5]
            ]
        };

        assert_ne!(m1, m2);
    }
}