use crate::ast::TypeKind;
use crate::vm::VirtualMachine;
use num_traits::*;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

pub mod functions;

pub trait Scalar: Copy + Num + BuiltInType {}
impl<T: Copy + Num + BuiltInType> Scalar for T {}

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
    pub fn comp_mul(mut self, other: Vector<T, N>) -> Vector<T, N> {
        for i in 0..self.elems.len() {
            self.elems[i] = self.elems[i] * other.elems[i];
        }
        self
    }
}

//TODO: Add a macro to automate this
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

/*#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Matrix<T, const N: usize, const M: usize> {
    pub rows: [Vector<T, N>; M]
}*/

pub trait BuiltInType {
    fn stack_size() -> usize;
    fn type_kind() -> TypeKind;
}

impl BuiltInType for f32 {
    fn stack_size() -> usize {
        std::mem::size_of::<f32>()
    }
    fn type_kind() -> TypeKind {
        TypeKind::F32
    }
}

impl<T: Scalar, const N: usize> BuiltInType for Vector<T, N> {
    fn stack_size() -> usize {
        std::mem::size_of::<T>()
    }
    fn type_kind() -> TypeKind {
        TypeKind::Vector(Box::new(T::type_kind()), N)
    }
}

use crate::glsl::BuiltInCallableGLSL;

pub trait BuiltInCallable: BuiltInCallableGLSL {
    fn ident(&self) -> &str;
    fn vm_impl(&self, vm: &mut VirtualMachine);
    fn return_type(&self) -> TypeKind;
    fn arg_types(&self) -> Vec<TypeKind>;
}

pub fn get_builtin_fn<'a>(
    id: &str,
    arg_types: &'a [TypeKind],
) -> Option<(usize, &'static dyn BuiltInCallable)> {
    for (i, f) in functions::FUNCTIONS.iter().enumerate() {
        if f.ident() == id && f.arg_types().as_slice() == arg_types {
            return Some((i, *f));
        }
    }

    None
}

pub fn call_builtin_fn(func_id: usize, vm: &mut VirtualMachine) {
    functions::FUNCTIONS[func_id].vm_impl(vm);
}
