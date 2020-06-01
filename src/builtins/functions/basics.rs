use crate::ast::TypeKind;
use crate::builtins::*;
use crate::vm::VirtualMachine;
use builtins::generate_builtin_fn;
use builtins::generate_glsl_impl_inline;

use crate::glsl::compiler::GenerateGLSL;
use crate::glsl::BuiltInCallableGLSL;

#[generate_builtin_fn("__op_binary_mul")]
fn BinMulFloatVec3(a: f32, v: Vec3) -> Vec3 {
    Vec3 {
        x: v.x * a,
        y: v.y * a,
        z: v.z * a,
    }
}

#[generate_glsl_impl_inline("BinMulFloatVec3")]
fn generate(a: &str, b: &str) -> String {
    format!("{} * {}", a, b)
}

#[generate_builtin_fn("__op_binary_mul")]
fn BinMulVec3Float(v: Vec3, a: f32) -> Vec3 {
    Vec3 {
        x: v.x * a,
        y: v.y * a,
        z: v.z * a,
    }
}

#[generate_glsl_impl_inline("BinMulVec3Float")]
fn generate(a: &str, b: &str) -> String {
    format!("{} * {}", a, b)
}

#[generate_builtin_fn("__op_binary_add")]
fn BinAddVec3Vec3(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

#[generate_glsl_impl_inline("BinAddVec3Vec3")]
fn generate(a: &str, b: &str) -> String {
    format!("{} + {}", a, b)
}

#[generate_builtin_fn("Vec3")]
fn Vec3Constructor(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[generate_glsl_impl_inline("Vec3Constructor")]
fn generate(a: &str, b: &str, c: &str) -> String {
    format!("vec3({}, {}, {})", a, b, c)
}

#[generate_builtin_fn("normalize")]
fn Vec3Normalize(a: Vec3) -> Vec3 {
    let len = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
    Vec3 {
        x: a.x / len,
        y: a.y / len,
        z: a.z / len,
    }
}

#[generate_glsl_impl_inline("Vec3Normalize")]
fn generate(a: &str) -> String {
    format!("normalize({})", a)
}

#[generate_builtin_fn("dot")]
fn Vec3Dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

#[generate_glsl_impl_inline("Vec3Dot")]
fn generate(a: &str, b: &str) -> String {
    format!("dot({}, {})", a, b)
}

#[generate_builtin_fn("__op_unary_neg")]
fn UnNegFloat(a: f32) -> f32 {
    -a
}

#[generate_glsl_impl_inline("UnNegFloat")]
fn generate(a: &str) -> String {
    format!("-{}", a)
}
