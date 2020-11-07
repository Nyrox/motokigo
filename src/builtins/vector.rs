use crate::ast::TypeKind;
use crate::builtins::{BuiltInType, Matrix, Scalar};
use num_traits::*;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector<T: Scalar, const N: usize>(Matrix<T, 1, N>);

impl<T: Scalar, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<T: Scalar + Debug, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.0.fmt(formatter)
    }
}

unsafe impl<T: 'static + Scalar, const N: usize> bytemuck::Pod for Vector<T, N> {}
unsafe impl<T: Scalar, const N: usize> bytemuck::Zeroable for Vector<T, N> {}

impl<T: Scalar, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(mut self, other: T) -> Self::Output {
        for i in 0..N {
            self.set_elem(i, self.get_elem(i) * other);
        }
        self
    }
}

impl<T: Scalar, const N: usize> Add<Self> for Vector<T, N> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..N {
            self.set_elem(i, self.get_elem(i) + other.get_elem(i));
        }
        self
    }
}

impl<T: Scalar, const N: usize> Sub<Self> for Vector<T, N> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..N {
            self.set_elem(i, self.get_elem(i) - other.get_elem(i));
        }
        self
    }
}

impl<T: Scalar, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(mut self, other: T) -> Self::Output {
        for i in 0..N {
            self.set_elem(i, self.get_elem(i) / other);
        }
        self
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn from_arr(elems: [T; N]) -> Self {
        Self(Matrix { rows: [elems] })
    }

    pub fn to_arr(self) -> [T; N] {
        self.0.rows[0]
    }

    pub fn get_elem(&self, n: usize) -> T {
        self.0.rows[0][n]
    }

    pub fn set_elem(&mut self, n: usize, v: T) {
        self.0.rows[0][n] = v;
    }

    pub fn lengthSquared(self) -> T {
        let mut sum = Zero::zero();
        for i in 0..N {
            sum = sum + self.get_elem(i) * self.get_elem(i);
        }
        sum
    }

    pub fn length(self) -> f32 {
        self.lengthSquared().to_f32().unwrap().sqrt()
    }

    pub fn dot(self, other: Self) -> T {
        let mut sum = Zero::zero();
        for i in 0..N {
            sum = sum + self.get_elem(i) * other.get_elem(i);
        }
        sum
    }
}

impl<T: Scalar + From<f32>, const N: usize> Vector<T, N> {
    pub fn normalize(self) -> Self {
        self / self.length().into()
    }
}

//TODO: Add a macro to automate this
impl<T: Scalar> Vector<T, 4> {
    pub fn x(self) -> T {
        self.get_elem(0)
    }
    pub fn y(self) -> T {
        self.get_elem(1)
    }
    pub fn z(self) -> T {
        self.get_elem(2)
    }
    pub fn w(self) -> T {
        self.get_elem(3)
    }

    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from_arr([x, y, z, w])
    }
}

impl<T: Scalar> Vector<T, 3> {
    pub fn x(self) -> T {
        self.get_elem(0)
    }
    pub fn y(self) -> T {
        self.get_elem(1)
    }
    pub fn z(self) -> T {
        self.get_elem(2)
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Self::from_arr([x, y, z])
    }
}

impl<T: Scalar> Vector<T, 2> {
    pub fn x(&self) -> T {
        self.get_elem(0)
    }
    pub fn y(&self) -> T {
        self.get_elem(1)
    }

    pub fn new(x: T, y: T) -> Self {
        Self::from_arr([x, y])
    }
}

pub type Vec4 = Vector<f32, 4>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec2 = Vector<f32, 2>;

impl<T: Scalar, const N: usize> BuiltInType for Vector<T, N> {
    fn stack_size() -> usize {
        std::mem::size_of::<T>() * N
    }
    fn type_kind() -> TypeKind {
        TypeKind::Vector(Box::new(T::type_kind()), N)
    }
}
