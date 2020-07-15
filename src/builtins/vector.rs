use crate::builtins::{Scalar, BuiltInType};
use crate::ast::TypeKind;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};
use num_traits::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector<T: Scalar, const N: usize> {
    pub elems: [T; N],
}

impl<T: Scalar, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<T: Scalar + Debug, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.elems.fmt(formatter)
    }
}

unsafe impl<T: 'static + Scalar, const N: usize> bytemuck::Pod for Vector<T, N> {}
unsafe impl<T: Scalar, const N: usize> bytemuck::Zeroable for Vector<T, N> {}

impl<T: Scalar, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(mut self, other: T) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] * other;
        }
        self
    }
}

impl<T: Scalar, const N: usize> Add<Self> for Vector<T, N> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] + other.elems[i];
        }
        self
    }
}

impl<T: Scalar, const N: usize> Sub<Self> for Vector<T, N> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] - other.elems[i];
        }
        self
    }
}

impl<T: Scalar, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(mut self, other: T) -> Self::Output {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] / other;
        }
        self
    }
}

impl<T: Scalar, const N: usize> Vector<T, N> {
    pub fn comp_mul(mut self, other: Self) -> Self {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] * other.elems[i];
        }
        self
    }

    pub fn lengthSquared(self) -> T {
        let mut sum = Zero::zero();
        for i in 0..self.elems.len() {
            sum = sum + self.elems[i] * self.elems[i];
        }
        sum
    }

    pub fn length(self) -> f32 {
        self.lengthSquared().to_f32().unwrap().sqrt()
    }

    pub fn dot(self, other: Self) -> T {
        let mut sum = Zero::zero();
        for i in 0..self.elems.len() {
            sum = sum + self.elems[i] * other.elems[i];
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
        self.elems[0]
    }
    pub fn y(self) -> T {
        self.elems[1]
    }
    pub fn z(self) -> T {
        self.elems[2]
    }
    pub fn w(self) -> T {
        self.elems[3]
    }

    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { elems: [x, y, z, w] }
    }
}

impl<T: Scalar> Vector<T, 3> {
    pub fn x(self) -> T {
        self.elems[0]
    }
    pub fn y(self) -> T {
        self.elems[1]
    }
    pub fn z(self) -> T {
        self.elems[2]
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { elems: [x, y, z] }
    }
}

impl<T: Scalar> Vector<T, 2> {
    pub fn x(&self) -> T {
        self.elems[0]
    }
    pub fn y(&self) -> T {
        self.elems[1]
    }

    pub fn new(x: T, y: T) -> Self {
        Self { elems: [x, y] }
    }
}

pub type Vec4 = Vector<f32, 4>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec2 = Vector<f32, 2>;

impl<T: Scalar, const N: usize> BuiltInType for Vector<T, N> {
    fn stack_size() -> usize {
        std::mem::size_of::<T>()
    }
    fn type_kind() -> TypeKind {
        TypeKind::Vector(Box::new(T::type_kind()), N)
    }
}