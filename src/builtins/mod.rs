use crate::{ast::TypeKind, vm::VirtualMachine};
use num_traits::*;

pub mod functions;
pub mod vector;
pub use vector::*;
pub mod matrix;
pub use matrix::*;

pub trait Scalar: Copy + Num + ToPrimitive + BuiltInType {}
impl<T: Copy + Num + ToPrimitive + BuiltInType> Scalar for T {}

pub trait BuiltInType {
	fn stack_size() -> usize;
	fn type_kind() -> TypeKind;
}

pub type Float = f32;
pub type Int = i32;

impl BuiltInType for f32 {
	fn stack_size() -> usize {
		std::mem::size_of::<f32>()
	}

	fn type_kind() -> TypeKind {
		TypeKind::F32
	}
}

impl BuiltInType for i32 {
	fn stack_size() -> usize {
		std::mem::size_of::<i32>()
	}

	fn type_kind() -> TypeKind {
		TypeKind::I32
	}
}

use crate::glsl::BuiltInCallableGLSL;

pub trait BuiltInCallable: BuiltInCallableGLSL {
	fn ident(&self) -> &str;
	fn vm_impl(&self, vm: &mut VirtualMachine);
	fn return_type(&self) -> TypeKind;
	fn arg_types(&self) -> Vec<TypeKind>;
}

pub fn get_builtin_fn<'a>(id: &str, arg_types: &'a [TypeKind]) -> Option<(usize, &'static dyn BuiltInCallable)> {
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
