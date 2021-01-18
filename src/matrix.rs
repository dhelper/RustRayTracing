use std::ops::Index;

pub struct Matrix4 {
    values: [[f64; 4]; 4]
}

pub struct Matrix3 {
    values: [[f64; 3]; 3]
}

pub struct Matrix2 {
    values: [[f64; 2]; 2]
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

impl Index<(usize, usize)> for Matrix2 {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix4;
    use crate::matrix::Matrix3;
    use crate::matrix::Matrix2;

    #[test]
    fn constructing_an_inspecting_a_4x4_matrix() {
        let m = Matrix4 {
            values: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        };

        assert_eq!(1.0, m.values[0][0]);
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
}