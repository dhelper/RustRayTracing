use std::ops::{Index, IndexMut};
use core::ops;
use crate::tuple::Tuple;

#[macro_export]
macro_rules! inc_by_1 {
    ($n:expr, $max_n:expr) =>
    {
        $n = ($n + 1) % $max_n;
    }
}

#[macro_export]
macro_rules! matrix {
    ($name:ident, $n:expr) =>
    {
        #[derive(Copy, Clone)]
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

        impl IndexMut<(usize, usize)> for $name {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
                &mut self.values[index.0][index.1]
            }
        }

        impl ops::Mul<$name> for $name {
            type Output = $name;

            fn mul(self, rhs: $name) -> Self::Output {
                let mut tmp: [[f64; $n]; $n] = Default::default();

                for row in 0..$n {
                    for col in 0..$n {
                        for index in 0..$n{
                            tmp[row][col] +=self[(row, index)] * rhs[(index, col)]
                        }
                    }
                }

                return $name {
                    values: tmp
                };
            }
        }

        impl $name {
            pub fn new(input: [[f64; $n]; $n]) -> Self{
                $name { values: input }
            }

            #[inline]
            pub fn identity_matrix() -> $name {
                let mut tmp: [[f64; $n]; $n] = Default::default();
                for index in 0..$n {
                    tmp[index][index] = 1.0;
                }
                $name { values: tmp }
            }

            pub fn transpose(self) -> $name {
                let mut tmp: [[f64; $n]; $n] = Default::default();

                for row in 0..$n {
                    for col in 0..$n {
                        tmp[col][row] = self.values[row][col];
                    }
                }

                return $name { values: tmp };
            }

            pub fn is_invertibile(self) -> bool{
                return self.determinant() != 0.0;
            }

            pub fn round(self) -> $name{
                let mut tmp: [[f64; $n]; $n] = Default::default();

                for row in 0..$n {
                    for col in 0..$n {
                        tmp[row][col] = (self.values[row][col] * 100000.0).round() /100000.0;
                    }
                }

                return $name { values: tmp };
            }
        }
    }
}

matrix!(Matrix4, 4);
matrix!(Matrix3, 3);
matrix!(Matrix2, 2);

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result: Tuple = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        for row in 0..4 {
            for col in 0..4 {
                result[row] += self[(row, col)] * rhs[col];
            }
        }

        return result;
    }
}

impl Matrix2 {
    fn determinant(self) -> f64 {
        return self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0];
    }
}

#[macro_export]
macro_rules! submatrix {
    ($n_prev:expr, $type:ident) =>{
        fn submatrix(self, delete_row: usize, delete_col: usize) -> $type {
            let mut tmp: [[f64;$n_prev]; $n_prev] = Default::default();
            let mut row_2 = 0;
            let mut col_2 = 0;

            for row in 0..self.values.len() {
                if row == delete_row {
                    continue;
                }
                for col in 0..self.values.len() {
                    if col == delete_col {
                        continue;
                    }

                    tmp[row_2][col_2] = self[(row, col)];

                    inc_by_1!(col_2, tmp.len())
                }

                inc_by_1!(row_2, tmp.len())
            }

            return $type::new(tmp);
        }

        pub fn minor(self, row: usize, col: usize) -> f64 {
            return self.submatrix(row, col).determinant();
        }

        pub fn cofactor(self, row: usize, col: usize) -> f64 {
            if (row + col) % 2 == 0 {
                return self.minor(row, col);
            }

            return self.minor(row, col) * -1.0;
        }

        fn determinant(self) -> f64 {
           let mut det:f64 = 0.0;

            for col in 0..self.values.len() {
                det += self.values[0][col] * self.cofactor(0, col);
            }

            return det;
        }
    }
}

impl Matrix3 {
    submatrix!(2, Matrix2);
}

impl Matrix4 {
    submatrix!(3, Matrix3);

    fn inverse(self) -> Matrix4 {
        let mut result: [[f64; 4]; 4] = Default::default();
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                result[col][row] = self.cofactor(row, col) / determinant;
            }
        }

        return Matrix4::new(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix2;
    use crate::tuple::Tuple;

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

    #[test]
    fn multiplying_two_4x4_matrices() {
        let m1 = Matrix4 {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]
        };

        let m2 = Matrix4 {
            values: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0]
            ]
        };

        let result = m1 * m2;

        let expected = Matrix4 {
            values: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_two_3x3_matrices() {
        let m1 = Matrix3 {
            values: [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        };

        let m2 = Matrix3 {
            values: [
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0]
            ]
        };

        let result = m1 * m2;

        let expected = Matrix3 {
            values: [
                [30.0, 36.0, 42.0],
                [66.0, 81.0, 96.0],
                [102.0, 126.0, 150.0],
            ]
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_two_2x2_matrices() {
        let m1 = Matrix2 {
            values: [
                [1.0, 2.0],
                [3.0, 4.0]
            ]
        };

        let m2 = Matrix2 {
            values: [
                [5.0, 6.0],
                [7.0, 8.0]
            ]
        };

        let result = m1 * m2;

        let expected = Matrix2 {
            values: [
                [19.0, 22.0],
                [43.0, 50.0]
            ]
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let m = Matrix4 {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        let t = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };

        let result = m * t;

        let expected = Tuple { x: 18.0, y: 24.0, z: 33.0, w: 1.0 };

        assert_eq!(expected, result);
    }

    #[test]
    fn multiplying_4x4_matrix_by_the_identity_matrix() {
        let m = Matrix4 {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        let result = m * Matrix4::identity_matrix();

        assert_eq!(m, result);
    }

    #[test]
    fn multiplying_3x3_matrix_by_the_identity_matrix() {
        let m = Matrix3 {
            values: [
                [1.0, 2.0, 3.0],
                [2.0, 4.0, 4.0],
                [8.0, 6.0, 4.0]
            ]
        };

        let result = m * Matrix3::identity_matrix();

        assert_eq!(m, result);
    }

    #[test]
    fn multiplying_2x2_matrix_by_the_identity_matrix() {
        let m = Matrix2 {
            values: [
                [1.0, 2.0],
                [2.0, 4.0],
            ]
        };

        let result = m * Matrix2::identity_matrix();

        assert_eq!(m, result);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let t = Tuple { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };

        let result = Matrix4::identity_matrix() * t;

        assert_eq!(t, result);
    }

    #[test]
    fn transposing_a_4x4_matrix() {
        let m = Matrix4::new(
            [[0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]]
        );

        let result = m.transpose();

        let expected = Matrix4::new(
            [[0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]]
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn transposing_a_3x3_matrix() {
        let m = Matrix3::new(
            [
                [0.0, 9.0, 3.0],
                [9.0, 8.0, 0.0],
                [1.0, 8.0, 5.0]
            ]
        );

        let result = m.transpose();

        let expected = Matrix3::new([
            [0.0, 9.0, 1.0],
            [9.0, 8.0, 8.0],
            [3.0, 0.0, 5.0]
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn transposing_a_2x2_matrix() {
        let m = Matrix2::new(
            [
                [0.0, 9.0],
                [9.0, 8.0]
            ]
        );

        let result = m.transpose();

        let expected = Matrix2::new([
            [0.0, 9.0],
            [9.0, 8.0]
        ]);

        assert_eq!(expected, result);
    }

    macro_rules! transpose_identity_tests {
        ($($name:ident: $type:ident,)*) => {
        $(
            #[test]
            fn $name() {
                let identity = $type::identity_matrix();

                let result = identity.transpose();

                assert_eq!($type::identity_matrix(), result);
            }
        )*
        }
    }

    transpose_identity_tests! {
        transpose_4x4_identity_matrix: Matrix4,
        transpose_3x3_identity_matrix: Matrix3,
        transpose_2x2_identity_matrix: Matrix2,
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = Matrix2::new([
            [1.0, 5.0],
            [-3.0, 2.0]
        ]);

        let result = m.determinant();

        assert_eq!(17.0, result);
    }

    macro_rules! submatrix_of_a_3x3_matrix_tests {
        ($($name:ident: $value:expr,)*) => {
             $(
            #[test]
            fn $name(){
                let m = Matrix3::new([
                            [1.0, 2.0, 3.0],
                            [4.0, 5.0, 6.0],
                            [7.0, 8.0, 9.0]
                        ]);

                let (row, col, expected) = $value;

                let result = m.submatrix(row, col);

                assert_eq!(expected, result);
            }
            )*
        }
    }

    submatrix_of_a_3x3_matrix_tests! {
        submatrix_of_a_3x3_matrix_0_0: (0,0, Matrix2::new([[5.0, 6.0],[8.0,9.0]])),
        submatrix_of_a_3x3_matrix_0_1: (0,1, Matrix2::new([[4.0, 6.0],[7.0,9.0]])),
        submatrix_of_a_3x3_matrix_0_2: (0,2, Matrix2::new([[4.0, 5.0],[7.0,8.0]])),
        submatrix_of_a_3x3_matrix_1_0: (1,0, Matrix2::new([[2.0, 3.0],[8.0,9.0]])),
        submatrix_of_a_3x3_matrix_1_1: (1,1, Matrix2::new([[1.0, 3.0],[7.0,9.0]])),
        submatrix_of_a_3x3_matrix_1_2: (1,2, Matrix2::new([[1.0, 2.0],[7.0,8.0]])),
        submatrix_of_a_3x3_matrix_2_0: (2,0, Matrix2::new([[2.0, 3.0],[5.0,6.0]])),
        submatrix_of_a_3x3_matrix_2_1: (2,1, Matrix2::new([[1.0, 3.0],[4.0,6.0]])),
        submatrix_of_a_3x3_matrix_2_2: (2,2, Matrix2::new([[1.0, 2.0],[4.0,5.0]])),
    }

    macro_rules! submatrix_of_a_4x4_matrix_tests {
        ($($name:ident: $value:expr,)*) => {
             $(
            #[test]
            fn $name(){
                let m = Matrix4::new([
                            [1.0, 2.0, 3.0, 4.0],
                            [5.0, 6.0, 7.0, 8.0],
                            [9.0, 10.0, 11.0, 12.0],
                            [13.0, 14.0, 15.0, 16.0]
                        ]);

                let (row, col, expected) = $value;

                let result = m.submatrix(row, col);

                assert_eq!(expected, result);
            }
            )*
        }
    }

    submatrix_of_a_4x4_matrix_tests! {
        submatrix_of_a_4x4_0_0: (0, 0, Matrix3::new([[6.0, 7.0, 8.0],[10.0, 11.0, 12.0], [14.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_0_1: (0, 1, Matrix3::new([[5.0, 7.0, 8.0],[9.0, 11.0, 12.0], [13.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_0_2: (0, 2, Matrix3::new([[5.0, 6.0, 8.0],[9.0, 10.0, 12.0], [13.0, 14.0, 16.0]])),
        submatrix_of_a_4x4_0_3: (0, 3, Matrix3::new([[5.0, 6.0, 7.0],[9.0, 10.0, 11.0], [13.0, 14.0, 15.0]])),
        submatrix_of_a_4x4_1_0: (1, 0, Matrix3::new([[2.0, 3.0, 4.0],[10.0, 11.0, 12.0], [14.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_1_1: (1, 1, Matrix3::new([[1.0, 3.0, 4.0],[9.0, 11.0, 12.0], [13.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_1_2: (1, 2, Matrix3::new([[1.0, 2.0, 4.0],[9.0, 10.0, 12.0], [13.0, 14.0, 16.0]])),
        submatrix_of_a_4x4_1_3: (1, 3, Matrix3::new([[1.0, 2.0, 3.0],[9.0, 10.0, 11.0], [13.0, 14.0, 15.0]])),
        submatrix_of_a_4x4_2_0: (2, 0, Matrix3::new([[2.0, 3.0, 4.0],[6.0, 7.0, 8.0], [14.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_2_1: (2, 1, Matrix3::new([[1.0, 3.0, 4.0],[5.0, 7.0, 8.0], [13.0, 15.0, 16.0]])),
        submatrix_of_a_4x4_2_2: (2, 2, Matrix3::new([[1.0, 2.0, 4.0],[5.0, 6.0, 8.0], [13.0, 14.0, 16.0]])),
        submatrix_of_a_4x4_2_3: (2, 3, Matrix3::new([[1.0, 2.0, 3.0],[5.0, 6.0, 7.0], [13.0, 14.0, 15.0]])),
        submatrix_of_a_4x4_3_0: (3, 0, Matrix3::new([[2.0, 3.0, 4.0],[6.0, 7.0, 8.0], [10.0, 11.0, 12.0]])),
        submatrix_of_a_4x4_3_1: (3, 1, Matrix3::new([[1.0, 3.0, 4.0],[5.0, 7.0, 8.0], [9.0, 11.0, 12.0]])),
        submatrix_of_a_4x4_3_2: (3, 2, Matrix3::new([[1.0, 2.0, 4.0],[5.0, 6.0, 8.0], [9.0, 10.0, 12.0]])),
        submatrix_of_a_4x4_3_3: (3, 3, Matrix3::new([[1.0, 2.0, 3.0],[5.0, 6.0, 7.0], [9.0, 10.0, 11.0]])),
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let m = Matrix3::new([
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ]);

        let matrix2 = m.submatrix(1, 0);
        let determinant2 = matrix2.determinant();

        assert_eq!(determinant2, m.minor(1, 0))
    }

    #[test]
    fn calculating_a_minor_of_a_4x4_matrix() {
        let m = Matrix4::new([
            [3.0, 5.0, 0.0, 1.0],
            [2.0, -1.0, -7.0, 2.0],
            [6.0, -1.0, 5.0, 3.0],
            [16.0, -11.0, 15.0, 13.0]
        ]);

        let matrix3 = m.submatrix(1, 0);
        let determinant3 = matrix3.determinant();

        assert_eq!(determinant3, m.minor(1, 0))
    }

    macro_rules! cofactor_of_a_3x3_matrix_tests {
        ($($name:ident: $value:expr,)*) => {
             $(
            #[test]
            fn $name(){
                let m = Matrix3::new([
                            [3.0, 5.0, 0.0],
                            [2.0, -1.0, -7.0],
                            [6.0, -1.0, 5.0]
                        ]);

                let (row, col, expected_sign) = $value;

                let result = m.cofactor(row, col);

                assert_eq!(expected_sign * m.minor(row, col), result);
            }
            )*
        }
    }

    cofactor_of_a_3x3_matrix_tests! {
        cofactor_of_a_3x3_matrix_0_0: (0, 0, 1.0),
        cofactor_of_a_3x3_matrix_0_1: (0, 1, -1.0),
        cofactor_of_a_3x3_matrix_0_2: (0, 2, 1.0),
        cofactor_of_a_3x3_matrix_1_0: (1, 0, -1.0),
        cofactor_of_a_3x3_matrix_1_1: (1, 1, 1.0),
        cofactor_of_a_3x3_matrix_1_2: (1, 2, -1.0),
        cofactor_of_a_3x3_matrix_2_0: (2, 0, 1.0),
        cofactor_of_a_3x3_matrix_2_1: (2, 1, -1.0),
        cofactor_of_a_3x3_matrix_2_2: (2, 2, 1.0),
    }

    macro_rules! cofactor_of_a_4x4_matrix_tests {
        ($($name:ident: $value:expr,)*) => {
             $(
            #[test]
            fn $name(){
                let m = Matrix4::new([
                            [3.0, 5.0, 0.0, 2.0],
                            [2.0, -1.0, -7.0, 3.0],
                            [6.0, -1.0, 5.0, 4.0],
                            [16.0, -11.0, 15.0, 5.0],
                        ]);

                let (row, col, expected_sign) = $value;

                let result = m.cofactor(row, col);

                assert_eq!(expected_sign * m.minor(row, col), result);
            }
            )*
        }
    }

    cofactor_of_a_4x4_matrix_tests! {
        cofactor_of_a_4x4_matrix_0_0: (0, 0, 1.0),
        cofactor_of_a_4x4_matrix_0_1: (0, 1, -1.0),
        cofactor_of_a_4x4_matrix_0_2: (0, 2, 1.0),
        cofactor_of_a_4x4_matrix_0_3: (0, 3, -1.0),
        cofactor_of_a_4x4_matrix_1_0: (1, 0, -1.0),
        cofactor_of_a_4x4_matrix_1_1: (1, 1, 1.0),
        cofactor_of_a_4x4_matrix_1_2: (1, 2, -1.0),
        cofactor_of_a_4x4_matrix_1_3: (1, 3, 1.0),
        cofactor_of_a_4x4_matrix_2_0: (2, 0, 1.0),
        cofactor_of_a_4x4_matrix_2_1: (2, 1, -1.0),
        cofactor_of_a_4x4_matrix_2_2: (2, 2, 1.0),
        cofactor_of_a_4x4_matrix_2_3: (2, 3, -1.0),
        cofactor_of_a_4x4_matrix_3_0: (3, 0, -1.0),
        cofactor_of_a_4x4_matrix_3_1: (3, 1, 1.0),
        cofactor_of_a_4x4_matrix_3_2: (3, 2, -1.0),
        cofactor_of_a_4x4_matrix_3_3: (3, 3, 1.0),
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let m = Matrix3::new([
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]
        ]);

        let result = m.determinant();

        assert_eq!(-196.0, result);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]
        ]);

        let result = m.determinant();

        assert_eq!(-4071.0, result);
    }

    #[test]
    fn testing_an_4x4_invertible_matrix_for_invertibility() {
        let m = Matrix4::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0]
        ]);

        assert_eq!(-2120.0, m.determinant());
        assert!(m.is_invertibile())
    }

    #[test]
    fn testing_an_3x3_invertible_matrix_for_invertibility() {
        let m = Matrix3::new([
            [6.0, 4.0, 4.0],
            [5.0, 5.0, 7.0],
            [4.0, -9.0, 3.0],
        ]);

        assert!(m.is_invertibile())
    }

    #[test]
    fn testing_an_2x2_invertible_matrix_for_invertibility() {
        let m = Matrix2::new([
            [6.0, 4.0, ],
            [5.0, 5.0, ],
        ]);

        assert!(m.is_invertibile())
    }

    #[test]
    fn testing_a_non_invertible_matrix_for_invertibility() {
        let m = Matrix4::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0]
        ]);

        assert_eq!(0.0, m.determinant());
        assert!(!m.is_invertibile())
    }

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
    fn multiplying_a_product_by_its_inverse(){
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
}