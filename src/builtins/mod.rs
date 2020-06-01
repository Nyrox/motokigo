use crate::ast::TypeKind;
use crate::vm::VirtualMachine;

pub mod functions;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl bytemuck::Pod for Vec3 {}
unsafe impl bytemuck::Zeroable for Vec3 {}

pub trait BuiltInType {
    fn stack_size() -> usize;
    fn type_kind() -> TypeKind;
}
impl BuiltInType for f32 {
    fn stack_size() -> usize {
        4
    }
    fn type_kind() -> TypeKind {
        TypeKind::F32
    }
}
impl BuiltInType for Vec3 {
    fn stack_size() -> usize {
        12
    }
    fn type_kind() -> TypeKind {
        TypeKind::Vec3
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
