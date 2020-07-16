use crate::builtins::{Scalar, BuiltInType, Vector};
use crate::ast::TypeKind;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};
use num_traits::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix<T: Scalar, const M: usize, const N: usize> {
    pub rows: [[T; N]; M]
}

impl<T: Scalar, const M: usize, const N: usize> Default for Matrix<T, M, N> {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<T: Scalar + Debug, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        formatter.write_str("Matrix [");
        for r in self.rows.iter() {
            formatter.write_str("\n\t");
            formatter
                .debug_list()
                .entries(r.iter())
                .finish()?;
        }
        formatter.write_str("\n]");
        Ok(())
    }
}

unsafe impl<T: 'static + Scalar, const M: usize, const N: usize> bytemuck::Pod for Matrix<T, M, N> {}
unsafe impl<T: Scalar, const M: usize, const N: usize> bytemuck::Zeroable for Matrix<T, M, N> {}

impl<T: Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(arr: [[T; N]; M]) -> Self {
        Self { rows: arr }
    }

    pub fn get_elem(self, row: usize, col: usize) -> T {
        self.rows[row][col]
    }

    pub fn get_row(self, row: usize) -> Vector<T, N> {
        Vector::from_arr(self.rows[row])
    }

    pub fn get_col(self, col: usize) -> Vector<T, M> {
        let mut result: [T; M] = [Zero::zero(); M];
        for i in 0..M {
            result[i] = self.rows[i][col];
        }
        Vector::from_arr(result)
    }

    pub fn transpose(self) -> Matrix<T, N, M> {
        let mut result: Matrix<T, N, M> = Default::default();
        for r in 0..N {
            for c in 0..M {
                result.rows[r][c] = self.rows[c][r];
            }
        }
        result
    }
}

impl<T: Scalar, const M: usize, const N: usize, const R: usize> Mul<Matrix<T, N, R>> for Matrix<T, M, N> {
    type Output = Matrix<T, M, R>;

    fn mul(self, other: Matrix<T, N, R>) -> Self::Output {
        let mut result: Self::Output = Default::default();
        for r in 0..M {
            for c in 0..R {
                result.rows[r][c] = self.get_row(r).dot(other.get_col(c));
            }
        }
        result
    }
}

impl<T: Scalar, const M: usize, const N: usize> PartialEq<Self> for Matrix<T, M, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if self.rows[i][j] != other.rows[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: Scalar, const M: usize, const N: usize> BuiltInType for Matrix<T, M, N> {
    fn stack_size() -> usize {
        std::mem::size_of::<T>()
    }
    fn type_kind() -> TypeKind {
        TypeKind::Vector(Box::new(T::type_kind()), N)
    }
}

pub type Mat2   = Matrix<f32, 2, 2>;
pub type Mat3   = Matrix<f32, 3, 3>;
pub type Mat4   = Matrix<f32, 4, 4>;
pub type Mat2x3 = Matrix<f32, 2, 3>;
pub type Mat2x4 = Matrix<f32, 2, 4>;
pub type Mat3x2 = Matrix<f32, 3, 2>;
pub type Mat3x4 = Matrix<f32, 3, 4>;
pub type Mat4x2 = Matrix<f32, 4, 2>;
pub type Mat4x3 = Matrix<f32, 4, 3>;

mod tests {
    #[test]
    fn test1() {
        let lhs = Matrix {
            rows: [
                [1, 2, 3, 4],
                [3, 4, 2, 5],
                [4, 3, 2, 1]
            ]
        };

        let rhs = Matrix {
            rows: [
                [5, 1, 2, 3],
                [4, 2, 3, 4],
                [1, 2, 3, 4],
                [2, 1, 5, 2]
            ]
        };

        let result = Matrix {
            rows: [
                [24, 15, 37, 31],
                [43, 20, 49, 43],
                [36, 15, 28, 34]
            ]
        };
        
        assert_eq!(lhs * rhs, result);
    }
}